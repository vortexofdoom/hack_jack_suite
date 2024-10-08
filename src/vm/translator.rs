use std::borrow::Cow;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::vec;

use anyhow::{bail, Result};

use super::{Comparison as Cmp, MemSegment as Seg, VmCommand};
use crate::asm::{Asm, Mode};
use asm_macro::asm;

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
            for _line in reader.lines().flatten() {
                // let cmd = match line.find("//") { //if let Some(i) = line.find("//") {
                //     Some(i) => &line[..i].trim(),
                //     _ => &line.trim(),
                // };
                // if !cmd.is_empty() {
                //     let vm_cmd = parse(&cmd).expect("could not parse command");
                //     writer.generate_asm(vm_cmd, true)?;
                // }
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

    fn generate_asm(&mut self, command: VmCommand<'a>, comment: bool) -> Result<()> {
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
            VmCommand::Push(seg, n) => match seg {
                Seg::Argument => self.push_segment(Asm::ARG, n),
                Seg::Local => self.push_segment(Asm::LCL, n),
                Seg::This => self.push_segment(Asm::THIS, n),
                Seg::That => self.push_segment(Asm::THAT, n),
                Seg::Static => self.push_value(format!("{}.{n}", self.filename), Mode::M),
                Seg::Pointer => {
                    self.push_value(if n == 0 { Asm::THIS } else { Asm::THAT }, Mode::M)
                }
                Seg::Temp => {
                    let reg = match n {
                        0 => Asm::R5,
                        1 => Asm::R6,
                        2 => Asm::R7,
                        3 => Asm::R8,
                        4 => Asm::R9,
                        5 => Asm::R10,
                        6 => Asm::R11,
                        7 => Asm::R12,
                        _ => bail!("Unsupported temp register {n}"),
                    };
                    self.push_value(reg, Mode::M)
                }
                Seg::Constant => self.push_constant(n),
            },
            VmCommand::Pop(seg, n) => match seg {
                Seg::Argument => self.pop_segment(Asm::ARG, n),
                Seg::Local => self.pop_segment(Asm::LCL, n),
                Seg::This => self.pop_segment(Asm::THIS, n),
                Seg::That => self.pop_segment(Asm::THAT, n),
                Seg::Static => self.pop_value(format!("{}.{n}", self.filename)),
                Seg::Pointer => self.pop_value(if n == 0 { Asm::THIS } else { Asm::THAT }),
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
                    "Get the return address from 5 slots before the current local segment and store it in R14"
                        @5
                        D=A
                        @LCL
                        A=M-D
                        D=M
                        @R14
                        M=D
                        ""
                        @SP
                        A=M-1
                        D=M
                        @ARG
                        A=M
                        M=D
                        D=A+1
                        @SP
                        M=D
                        ""
                        @LCL
                        D=M-1
                        @R13
                        AM=D
                    "restore saved that segment"
                        D=M
                        @THAT
                        M=D
                    "restore saved this segment"
                        @R13
                        AM=M-1
                        D=M
                        @THIS
                        M=D
                    "restore saved argument segment"
                        @R13
                        AM=M-1
                        D=M
                        @ARG
                        M=D
                    "restore saved local segment"
                        @R13
                        AM=M-1
                        D=M
                        @LCL
                        M=D
                    "jump to the saved return address"
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
        let end_comp = format!("END_COMP{counter}");

        // Computes the difference between the two values at the top of the stack
        self.binary_op(asm!(MD = M - D));

        self.asm.extend(vec![
            asm!(@end_comp),
            match comparison {
                Cmp::EQ => asm!(D;JNE),
                Cmp::GT => asm!(D;JLE),
                Cmp::LT => asm!(D;JGE),
                // Unofficial
                Cmp::LE => asm!(D;JGT),
                Cmp::GE => asm!(D;JLT),
                Cmp::NE => asm!(D;JEQ),
            },
        ]);

        self.asm.extend(asm![
            D=D+1
        (end_comp)
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
    pub fn push_segment(&mut self, segment: impl Display, n: i16) {
        self.segment(segment, n);

        self.asm.extend(asm![
            A=D+M
            D=M
        ]);

        self.push();
    }

    pub fn segment(&mut self, segment: impl Display, n: i16) {
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
    fn push_value<T: Display + Into<Asm<'a>>>(&mut self, var: T, mode: Mode) {
        self.asm.push(var.into());
        self.asm.push(match mode {
            Mode::A => asm!(D = A),
            Mode::M => asm!(D = M),
        });

        self.push();
    }

    fn push_constant(&mut self, var: i16) {
        match var {
            // For supported constants we can just update the top of the stack directly
            // This saves 2 instructions
            // This formulation is so I don't have to remember to update this if we change to push-optimized operations
            v @ -1..=1 => {
                // Make sure this optimization is always documented, even if not all commands are added as comments
                self.asm.push(asm!("push constant {v}"));
                self.push();
                // Get the last entry in the asm vector
                let idx = self.asm.len() - 1;
                self.asm[idx] = match v {
                    // TODO: Add support for -2 under optimization
                    -1 => asm!(M = -1),
                    0 => asm!(M = 0),
                    1 => asm!(M = 1),
                    _ => unreachable!(),
                }
            }
            v => {
                // If the constant to be pushed is negative, we can use an A instruction and a bitwise negation to push it
                // This works for even -32768 which is why we do it instead of arithmetic negation
                // `push constant n`
                // `neg/not`
                if var < 0 {
                    self.asm.extend([Asm::from(!v), asm!(D = !A)])
                } else {
                    self.asm.extend([Asm::from(v), asm!(D = A)])
                }
                // Then we push it to the stack
                self.push();
            }
        }
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

    fn pop_value<T: Display + Clone>(&mut self, var: T)
    where
        Asm<'a>: From<T>,
    {
        self.asm.extend(asm![
            @SP
            AM=M-1
            D=M
            @var
            M=D
        ]);
    }

    fn def_label(&mut self, label: String) {
        self.asm.push(Asm::Label(label.into()));
    }

    fn goto(&mut self, label: String) {
        self.asm.extend(asm![
            @label
            0;JMP
        ]);
    }

    fn if_goto(&mut self, label: String) {
        self.asm.extend(asm![
            @SP
            AM=M-1
            D=M
            @label
            D;JNE
        ]);
    }

    fn func(&mut self, fn_name: &str, n_vars: i16) {
        self.curr_func = String::from(fn_name);
        self.asm.extend(asm![
        ("{fn_name}")
            @n_vars
            D=A
            @SP
            M=D+M
        ("{fn_name}$LocalLoop")
            @"{fn_name}$LocalLoopEnd"
            D=D-1;JLT
            @LCL
            A=D+M
            M=0
            @"{fn_name}$LocalLoop"
            0;JMP
        ("{fn_name}$LocalLoopEnd")
        ]);
    }

    fn call_func(&mut self, function: &'a str, n_args: i16) {
        let return_label = format!("{}.ret${}", self.filename, self.call_count);
        self.call_count += 1;

        // Save return addr
        // This has to be done separately for each call
        self.push_value(asm!(@return_label), Mode::A);
        match n_args {
            0 => self.asm.extend(asm![
                @R14
                M=0
            ]),
            1 => self.asm.extend(asm![
                @R14
                M=1
            ]),
            a => self.asm.extend(asm![
                @a
                D=A
                @R14
                M=D
            ]),
        }
        self.asm.extend(asm![
            @n_args
            D=A
        ]);

        // TODO: The rest can be put into a assembly subroutine
        // We have to store n_args in a temp register
        // Save current stack frame
        self.asm.push(asm!(("$$Call")));
        self.push_value("LCL", Mode::M);
        self.push_value("ARG", Mode::M);
        self.push_value("THIS", Mode::M);
        self.push_value("THAT", Mode::M);
        self.asm.extend(asm![
            //@n_args
            //D=A
            @R14
            D=M
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
            @function
            0;JMP
        (return_label)
        ])
    }
}
