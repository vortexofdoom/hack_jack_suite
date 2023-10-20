use std::borrow::Cow;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::vec;

use anyhow::{anyhow, bail, Result};

use super::{Comparison as Cmp, MemSegment as Seg, VmCommand};
use crate::asm::{Asm, Instruction, Mode};
use asm_macro::asm;

pub fn parse(cmd: &str) -> Result<VmCommand> {
    //asm.push(code_writer::comment(cmd)); // comment with original vm command, stored separately so it can be skipped
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let command = match parts.len() {
        1 => match parts[0] {
            "add" => VmCommand::Add,
            "sub" => VmCommand::Sub,
            "neg" => VmCommand::Neg,
            "eq" => VmCommand::Compare(Cmp::Eq),
            "gt" => VmCommand::Compare(Cmp::GT),
            "lt" => VmCommand::Compare(Cmp::LT),
            "and" => VmCommand::And,
            "or" => VmCommand::Or,
            "not" => VmCommand::Not,
            "return" => VmCommand::Return,
            _ => bail!("No one word command \"{cmd}\""),
        },
        2 => match parts[0] {
            "label" => VmCommand::Label(parts[1]),
            "goto" => VmCommand::Goto(parts[1]),
            "if-goto" => VmCommand::IfGoto(parts[1]),
            _ => bail!("No two word command \"{cmd}\""),
        },
        3 => {
            let arg = parts[2]
                .parse::<i16>()
                .map_err(|_| anyhow!("{} is not a valid 16 bit integer", parts[2]))?;

            match (parts[0], parts[1]) {
                ("push", "local") => VmCommand::Push(Seg::Local, arg),
                ("pop", "local") => VmCommand::Pop(Seg::Local, arg),

                ("push", "argument") => VmCommand::Push(Seg::Argument, arg),
                ("pop", "argument") => VmCommand::Pop(Seg::Argument, arg),

                ("push", "this") => VmCommand::Push(Seg::This, arg),
                ("pop", "this") => VmCommand::Pop(Seg::This, arg),

                ("push", "that") => VmCommand::Push(Seg::That, arg),
                ("pop", "that") => VmCommand::Pop(Seg::That, arg),

                ("push", "constant") => VmCommand::Push(Seg::Constant, arg),

                ("push", "static") => VmCommand::Push(Seg::Static, arg),
                ("pop", "static") => VmCommand::Pop(Seg::Static, arg),

                ("push", "pointer") => VmCommand::Push(Seg::Pointer, arg),
                ("pop", "pointer") => VmCommand::Pop(Seg::Pointer, arg),

                ("push", "temp") => VmCommand::Push(Seg::Temp, arg),
                ("pop", "temp") => VmCommand::Pop(Seg::Temp, arg),

                ("function", _) => VmCommand::Function(parts[1], arg),
                ("call", _) => VmCommand::Call(parts[1], arg),

                _ => bail!("No three word command \"{cmd}\""),
            }
        }
        _ => bail!("\"{cmd}\" is not a valid VM command"),
    };
    Ok(command)
}

fn translate_vm(bootstrap: bool) -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<PathBuf> = vec![];
    let file_path = Path::new(&args[1]);
    let filename = file_path.file_stem().unwrap().to_str().unwrap();
    if file_path.is_dir() {
        for entry in file_path.read_dir().unwrap() {
            if let Some(x) = entry.as_ref().unwrap().path().extension() {
                if x.to_str().unwrap() == "vm" {
                    files.push(entry.as_ref().unwrap().path())
                }
            }
        }
    } else if let Some("vm") = file_path.extension().unwrap().to_str() {
        files.push(file_path.to_path_buf())
    }
    let mut writer = VmTranslator::new(filename, bootstrap);
    for file in files {
        if let Ok(f) = File::open(&file) {
            writer.set_filename(file.file_stem().unwrap().to_str().unwrap());
            let reader = BufReader::new(f);
            for line in reader.lines().flatten() {
                let cmd = line
                    .find("//")
                    .map(|i| &line[..i])
                    .unwrap_or(&line)
                    .trim()
                    .to_string();
                if !cmd.is_empty() {
                    let vm_cmd = parse(&cmd).expect("could not parse command");
                    writer.generate_asm(vm_cmd, true)?;
                }
            }
        }
    }
    Ok(())
    //writer.flush();
}

struct VmTranslator<'a> {
    filename: String,
    curr_func: String,
    comp_count: i16,
    call_count: i16,
    return_written: bool,
    asm: Vec<Asm<'a>>,
}

impl<'a> VmTranslator<'a> {
    pub fn new(filename: &str, bootstrap: bool) -> Self {
        let asm = if bootstrap {
            Vec::from(asm![
                @256
                D=A
                @SP
                M=D
                // Need to fine tune
            "call Sys.init"
                @"Sys.init"
                0;JMP
            ])
        } else {
            vec![]
        };

        Self {
            filename: filename.to_string(),
            curr_func: format!("${filename}$"),
            comp_count: 0,
            call_count: 0,
            return_written: false,
            asm,
        }
    }

    fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    /// Naively generates assembly on demand per VM Command.
    fn generate_asm(&mut self, command: VmCommand, comment: bool) -> Result<()> {
        if comment {
           self.asm.push(asm!("{command}"));
        }
        
        match command {
            VmCommand::Add => self.binary_op(asm!(M = D + M)),
            VmCommand::Sub => self.binary_op(asm!(M = M - D)),
            VmCommand::Neg => self.unary_op(asm!(M = -M)),
            VmCommand::Compare(comp) => self.comparison(comp),
            VmCommand::And => self.binary_op(asm!(M = D & M)),
            VmCommand::Or => self.binary_op(asm!(M = D | M)),
            VmCommand::Not => self.unary_op(asm!(M = !M)),
            VmCommand::Push(seg, n) => {
                match seg {
                    Seg::Argument => self.push_segment("ARG", n),
                    Seg::Local => self.push_segment("LCL", n),
                    Seg::This => self.push_segment("THIS", n),
                    Seg::That => self.push_segment("THAT", n),

                    Seg::Static => self.push_value(format!("{}.{n}", self.filename), Mode::M),
                    Seg::Pointer => self.push_value(if n == 0 { "THIS" } else { "THAT" }, Mode::M), // could probably just change this to n + 3
                    Seg::Temp => self.push_value(asm!(@"R{n}"), Mode::M),
                    Seg::Constant => self.push_constant(n),
                }
            }
            VmCommand::Pop(seg, n) => match seg {
                Seg::Argument => self.pop_segment("ARG", n),
                Seg::Local => self.pop_segment("LCL", n),
                Seg::This => self.pop_segment("THIS", n),
                Seg::That => self.pop_segment("THAT", n),
                Seg::Static => self.pop_value(format!("{}.{n}", self.filename)),
                Seg::Pointer => self.pop_value(if n == 0 { "THIS" } else { "THAT" }),
                Seg::Temp => self.pop_value(Asm::At(Cow::Owned(format!("R{}", n + 5)))),
                _ => bail!("cannot pop to constant"),
            },
            VmCommand::Label(l) => self.def_label(format!("{}${}", self.curr_func, l)),
            VmCommand::Goto(l) => self.goto(format!("{}${}", self.curr_func, l)),
            VmCommand::IfGoto(l) => self.if_goto(format!("{}${}", self.curr_func, l)),
            VmCommand::Function(f, n) => self.func(f, n),
            VmCommand::Call(f, n) => self.call_func(f, n),

            VmCommand::Return => {
                if self.return_written {
                    self.asm.extend(asm![
                        @"$$RETURN"
                        0;JMP
                    ])
                } else {
                    self.asm.extend(asm![
                    "Shared return subroutine"
                    ("$$RETURN")
                        @5
                        D=A
                        @LCL
                        A=M-D
                        D=M
                        @R14
                        M=D

                        @SP
                        A=M-1
                        D=M
                        @ARG
                        A=M
                        M=D
                        D=A+1
                        @SP
                        M=D

                        @LCL
                        D=M-1
                        @R13
                        AM=D

                        D=M
                        @THAT
                        M=D

                        @R13
                        AM=M-1
                        D=M
                        @THIS
                        M=D

                        @R13
                        AM=M-1
                        D=M
                        @ARG
                        M=D

                        @R13
                        AM=M-1
                        D=M
                        @LCL
                        M=D

                        @R14
                        A=M
                        0;JMP
                    ])
                }
            }
        }
        Ok(())
    }

    fn unary_op(&mut self, last_line: Asm<'a>) {
        self.asm.extend(asm![
            @SP
            A=M-1
        ]);

        self.asm.push(last_line);
    }

    fn comparison(&mut self, comparison: Cmp) {
        // making our comp_count into a simple identifier for formatting with the macro more easily
        let counter = self.comp_count;
        self.comp_count += 1;

        // Computes the difference between the two values at the top of the stack
        self.binary_op(asm!(MD = M - D));

        self.asm.extend(vec![
            asm!(@"END_COMP{counter}"),
            match comparison {
                // jumping if comparison is false
                Cmp::Eq => asm!(D;JNE),
                Cmp::GT => asm!(D;JLE),
                Cmp::LT => asm!(D;JGE),
            },
        ]);

        self.asm.extend(asm![
            D=D+1
        ("END_COMP{counter}")
            @SP
            A=M-1
            M=M-D
        ]);
    }

    // add, sub, and, or, and start of comparisons
    fn binary_op(&mut self, last_line: Asm<'a>) {
        self.asm.extend(
            asm![
                @SP
                AM=M-1
                D=M
                A=A-1
            ]
            .into_iter()
            .chain(std::iter::once(last_line)),
        );
    }

    // local, argument, this, that
    pub fn push_segment<T: Display>(&mut self, segment: T, n: i16) {
        self.segment(segment, n);

        self.asm.extend(asm![
            A=D+M
            D=M
        ]);

        self.push();
    }

    pub fn segment<T: Display>(&mut self, segment: T, n: i16) {
        self.asm
            .extend([Asm::from(n), asm!(D = A), asm!(@"{segment}")]);
    }

    pub fn pop_segment<T: Display>(&mut self, segment: T, n: i16) {
        self.segment(segment, n);

        self.asm.extend(asm![
            D=D+M
            @SP
            AM=M-1
            D=D+M
            A=D-M
            M=D-A
        ]);
    }
    // static, pointer, constant (push only)
    fn push_value<T: Display>(&mut self, var: T, mode: Mode) {
        self.asm.push(asm!(@"{var}"));
        self.asm.push(match mode {
            Mode::A => asm!(D = A),
            Mode::M => asm!(D = M),
        });

        self.push();
    }

    fn push_constant(&mut self, var: i16) {
        // If the constant to be pushed is negative, we can use an A instruction and a bitwise negation to push it
        // This works for even `i16::MIN` which is why we do it instead of arithmetic negation
        // If readability is important, -32768 could be special cased
        if var < 0 {
            self.asm.extend([Asm::from(!var), asm!(D = !A)]);
        } else {
            self.asm.extend([Asm::from(var), asm!(D = A)]);
        }

        // Then we push it to the stack
        self.push()
    }

    // Helper function to reduce code rewriting if we want to fine-tune the generated assembly
    fn push(&mut self) {
        self.asm.extend(asm![
            @SP
            M=M+1
            A=M-1
            M=D
        ])
    }

    fn pop_value<T: Display>(&mut self, var: T) {
        self.asm.extend(asm![
            @SP
            AM=M-1
            D=M
            @"{var}"
            M=D
        ]);
    }

    fn def_label(&mut self, label: String) {
        self.asm.push(asm!(("{label}")));
    }

    fn goto(&mut self, label: String) {
        self.asm.extend(asm![
            @"{label}"
            0;JMP
        ]);
    }

    fn if_goto(&mut self, label: String) {
        self.asm.extend(asm![
            @SP
            AM=M-1
            D=M
            @"{label}"
            D;JNE
        ]);
    }

    fn func(&mut self, fn_name: &str, n_vars: i16) {
        self.curr_func = String::from(fn_name);
        self.asm.extend(asm![
        ("{fn_name}")
            @"{n_vars}"
            D=A
            @SP
            AM=D+M
            D=D-1
        ("{fn_name}$LocalLoop")
            @"{fn_name}$LocalLoopEnd"
            D;JLT
            @LCL
            A=D+M
            M=0
            @"{fn_name}$LocalLoop"
            D=D-1;JMP
        ("{fn_name}$LocalLoopEnd")
        ]);
    }

    fn call_func(&mut self, function: &str, n_args: i16) {
        let return_label = format!("{}.ret${}", self.filename, self.call_count);
        self.call_count += 1;

        // Save return addr
        // Might want to make this explicitly save in a virtual register
        self.push_value(&return_label, Mode::A);
        // Save current stack frame
        self.push_value("LCL", Mode::M);
        self.push_value("ARG", Mode::M);
        self.push_value("THIS", Mode::M);
        self.push_value("THAT", Mode::M);

        self.asm.extend(asm![
            @"{n_args}"
            D=A
            @5
            D=D+A
            @SP
            D=M-D
            @ARG
            M=D
            @SP
            D=M
            @LCL
            M=D
            @"{function}"
            0;JMP
        ("{return_label}")
        ])
    }
}
