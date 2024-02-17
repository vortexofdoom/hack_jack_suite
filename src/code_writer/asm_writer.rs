use crate::tokens::vm_commands::{Comparison, MemSegment as M, VmCommand};
use asm_macro::asm;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Write};
use std::path::Path;
use std::str::FromStr;
use std::stringify;

use crate::asm::{Asm, Dest, Instruction, Jump, ValidComp};

pub struct AsmWriter {
    filename: String,
    writer: BufWriter<Cursor<Vec<u8>>>,
    curr_func: String,
    comp_count: i16,
    call_count: i16,
    return_written: bool,
}

enum Mode {
    A,
    M,
}

// const BOOTSTRAP: [Asm; 5] = [
//     Asm::addr(256),
//     Asm::c_inst("D=A"),
//     Asm::SP,
//     Asm::c_inst("M=D"),
//     Asm::comment("Call sys.init"),
// Asm::LCL,
// Asm::c_inst("D=M"),
// Asm::SP,
// Asm::c_inst("M=M+1"),
// Asm::c_inst("A=M+1"),
// Asm::c_inst("M=D"),
//];
//     Instruction::Asm(Asm::from_bits(256).unwrap()),
//     Instruction::Asm(())
//     Asm::from_str("D=A").unwrap(),
//     Asm::from_bits(0).unwrap(),
//     Asm::from_str("M=D").unwrap(),
//     Asm::from_str("@LCL").unwrap(),
//     Asm::from_str("D=M").unwrap(),
//     Asm::from_bits(0).unwrap(),
//     Asm::from_str("M=M+1").unwrap(),
//     Asm::from_str("A=M-1").unwrap(),
//     Asm::from_str("M=D").unwrap(),

//     @ARG
//     Asm::from_str("D=M").unwrap(),
//     @SP
//     ("M=M+1").unwrap(),
//     A=M-1
//     M=D

//     @THIS
//     D=M
//     0
//     M=M+1
//     A=M-1
//     M=D

//     @THAT
//     D=M
//     @SP
//     M=M+1
//     A=M-1
//     M=D

//     0
//     D=A
//     5
//     D=D+A
//     @SP
//     D=M-D
//     @ARG
//     M=D
//     0
//     D=M
//     @LCL
//     M=D
//     @Sys.init
//     0;JMP
// ];

// struct AsmFile {
//     filename: String,
//     buf: String,
// }

// const BOOTSTRAP: [Asm; 7] = asm![
//     "Bootstrap code"
//     @""
//     @256
//     D=A
//     @SP
//     M=D
//     "call Sys.init"
// ];

impl AsmWriter {
    pub fn new(filename: &str, bootstrap: bool) -> Self {
        let mut output = Vec::new();
        //File::create(Path::new(filename).with_extension("asm")).expect("could not create file");
        let mut writer = BufWriter::new(Cursor::new(output));
        if bootstrap {
            let call_sys_init = call_func("Sys.init", 0, String::from("Sys.init never returns"));
            write!(
                writer,
                "\
// bootstrap code
    @256
    D=A
    @SP
    M=D
// call sys_init
    {call_sys_init}
    "
            )
            .expect("failed to write bootstrap code");
        };
        AsmWriter {
            filename: filename.to_string(),
            writer,
            curr_func: format!("${filename}$"),
            comp_count: 0,
            call_count: 0,
            return_written: false,
        }
    }

    pub fn set_file_name(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }

    // comment
    pub fn comment(&mut self, comment: &str) {
        write!(
            self.writer,
            "
// {comment}
    "
        )
        .expect("failed to insert comment");
    }

    //#[allow(overflowing_literals)]
    pub fn generate_code(&mut self, command: VmCommand, comment: bool) {
        if comment {
            write!(self.writer, "// {command}").expect("failed to write comment");
        }
        let asm: String;
        match command {
            VmCommand::Add => asm = binary_op("M=D+M"),
            VmCommand::Sub => asm = binary_op("M=M-D"),
            VmCommand::Neg => asm = unary_op('-'),
            VmCommand::Compare(comp) => {
                asm = comparison(comp, self.comp_count);
                self.comp_count += 1;
            }
            VmCommand::And => asm = binary_op("M=D&M"),
            VmCommand::Or => asm = binary_op("M=D|M"),
            VmCommand::Not => asm = unary_op('!'),
            VmCommand::Push(seg, n) => {
                asm = match seg {
                    M::Argument => push_segment("ARG", n),
                    M::Local => push_segment("LCL", n),
                    M::This => push_segment("THIS", n),
                    M::That => push_segment("THAT", n),
                    M::Static => push_value(format!("{}.{n}", self.filename), false),
                    M::Pointer => push_value(if n == 0 { "THIS" } else { "THAT" }, false), // could probably just change this to n + 3
                    M::Temp => push_value(n + 5, false),
                    M::Constant => push_value(n, true),
                }
            }
            VmCommand::Pop(seg, n) => {
                asm = match seg {
                    M::Argument => pop_segment("ARG", n),
                    M::Local => pop_segment("LCL", n),
                    M::This => pop_segment("THIS", n),
                    M::That => pop_segment("THAT", n),
                    M::Static => pop_value(format!("{}.{n}", self.filename)),
                    M::Pointer => pop_value(if n == 0 { "THIS" } else { "THAT" }),
                    M::Temp => pop_value(n + 5),
                    _ => String::from("cannot pop to constant"),
                }
            }
            VmCommand::Label(l) => asm = def_label(format!("{}${}", self.curr_func, l)),
            VmCommand::Goto(l) => asm = goto(format!("{}${}", self.curr_func, l)),
            VmCommand::IfGoto(l) => asm = if_goto(format!("{}${}", self.curr_func, l)),
            VmCommand::Function(f, n) => {
                self.curr_func = f.to_string();
                asm = func(f, n);
            }
            VmCommand::Call(f, n) => {
                let return_label = format!("{}.ret${}", self.filename, self.call_count);
                asm = call_func(f, n, return_label);
                self.call_count += 1;
            }
            VmCommand::Return => {
                if self.return_written {
                    asm = return_func();
                } else {
                    asm = write_return();
                    self.return_written = true;
                }
            }
        };
        write!(self.writer, "{asm}").expect("failed to write command to asm file");
    }
}

// not and neg
fn unary_op(operator: char) -> String {
    format!(
        "\
    @SP
    A=M-1
    M={operator}M
    "
    )
}

// add, sub, and, or, and start of comparisons
fn binary_op(last_line: &str) -> String {
    format!(
        "\
    @SP
    AM=M-1
    D=M
    A=A-1
    {last_line}
    "
    )
}

// eq, gt, lt
fn comparison(comparison: Comparison, counter: i16) -> String {
    let comp_str = match comparison {
        // jumping if comparison is false
        Comparison::Eq => "NE",
        Comparison::GT => "LE",
        Comparison::LT => "GE",
    };
    
    binary_op("MD=M-D")
        + &format!(
            "\
    @END_COMP{counter}
    D;J{comp_str}
    D=D+1
(END_COMP{counter})
    @SP
    A=M-1
    M=M-D
    "
        )
}

// local, argument, this, that
pub fn push_segment<T>(segment: T, n: i16) -> String
where
    T: Display,
{
    format!(
        "\
    @{n}
    D=A
    @{segment}
    A=D+M
    D=M
    @SP
    M=M+1
    A=M-1
    M=D
    "
    )
}
pub fn pop_segment(segment: impl Display, n: i16) -> String {
    format!(
        "\
    @{n}
    D=A
    @{segment}
    D=D+M
    @SP
    AM=M-1
    D=D+M
    A=D-M
    M=D-A
    "
    )
}
// static, pointer, constant (push only)
fn push_value(var: impl Display, use_a_over_m: bool) -> String {
    let comp_a_or_m = if use_a_over_m { asm!(D=A) } else { asm!(D=M) };
    format!(
        "\
    @{var}
    D={comp_a_or_m}
    @SP
    M=M+1
    A=M-1
    M=D
    "
    )
}

fn push_value_<T: Display>(var: T, mode: Mode) {}

fn pop_value<T>(var: T) -> String
where
    T: Display,
{
    format!(
        "\
    @SP
    AM=M-1
    D=M
    @{var}
    M=D
    "
    )
}

fn def_label(label: String) -> String {
    format!(
        "\
    ({label})
    "
    )
}

fn goto(label: String) -> String {
    format!(
        "\
    @{label}
    0;JMP
    "
    )
}

fn if_goto(label: String) -> String {
    format!(
        "\
    @SP
    AM=M-1
    D=M
    @{label}
    D;JNE
    "
    )
}

fn func(fn_name: &str, n_vars: i16) -> String {
    format!(
        "\
({fn_name})
    @{n_vars}
    D=A
    @SP
    M=D+M
    D=D-1
({fn_name}$LocalLoop)
    @{fn_name}$LocalLoopEnd
    D;JLT
    @LCL
    A=D+M
    M=0
    @{fn_name}$LocalLoop
    D=D-1;JMP
({fn_name}$LocalLoopEnd)
    "
    )
}

fn call_func(function: &str, n_args: i16, return_label: String) -> String {
    let saved_return_addr = push_value(&return_label, true);
    let saved_lcl = push_value("LCL", false);
    let saved_arg = push_value("ARG", false);
    let saved_this = push_value("THIS", false);
    let saved_that = push_value("THAT", false);

    format!(
        "\
    {saved_return_addr}
    {saved_lcl}
    {saved_arg}
    {saved_this}
    {saved_that}
    @{n_args}
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
    @{function}
    0;JMP
({return_label})
    "
    )
}

fn return_func() -> String {
    String::from(
        "\
    @$$RETURN
    0;JMP
    ",
    )
}

fn write_return() -> String {
    String::from(
        "\
// Shared return subroutine
($$RETURN)
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
    ",
    )
}
