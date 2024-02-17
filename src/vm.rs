pub mod translator;

//use std::borrow::Cow;

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VmCommand<'a> {
    // Arithmetic
    Add,
    Sub,
    Neg,
    Compare(Comparison),
    And,
    Or,
    Not,
    //mem access
    Push(MemSegment, i16),
    Pop(MemSegment, i16),
    // Branching
    Label(&'a str),
    Goto(&'a str),
    IfGoto(&'a str),
    // Function
    Function(&'a str, i16),
    Call(&'a str, i16),
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemSegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparison {
    EQ,
    GT,
    LT,
    // Unofficial
    LE,
    GE,
    NE,
}

impl std::fmt::Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EQ => write!(f, "eq"),
            Self::GT => write!(f, "gt"),
            Self::LT => write!(f, "lt"),
            // Unofficial
            Self::LE => write!(f, "le"),
            Self::GE => write!(f, "ge"),
            Self::NE => write!(f, "ne"),
        }
    }
}

impl std::fmt::Display for MemSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Argument => write!(f, "argument"),
            Self::This => write!(f, "this"),
            Self::That => write!(f, "that"),
            Self::Constant => write!(f, "constant"),
            Self::Static => write!(f, "static"),
            Self::Pointer => write!(f, "pointer"),
            Self::Temp => write!(f, "temp"),
        }
    }
}

impl std::fmt::Display for VmCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmCommand::Add => write!(f, "add"),
            VmCommand::Sub => write!(f, "sub"),
            VmCommand::Neg => write!(f, "neg"),
            VmCommand::Compare(cmp) => write!(f, "{cmp}"),
            VmCommand::And => write!(f, "and"),
            VmCommand::Or => write!(f, "or"),
            VmCommand::Not => write!(f, "not"),
            VmCommand::Push(seg, arg) => write!(f, "push {seg} {arg}"),
            VmCommand::Pop(seg, arg) => write!(f, "pop {seg} {arg}"),
            VmCommand::Label(label) => write!(f, "label {label}"),
            VmCommand::Goto(label) => write!(f, "goto {label}"),
            VmCommand::IfGoto(label) => write!(f, "if-goto {label}"),
            VmCommand::Function(func, n) => write!(f, "function {func} {n}"),
            VmCommand::Call(func, n) => write!(f, "call {func} {n}"),
            VmCommand::Return => write!(f, "return"),
        }
    }
}

pub fn parse(cmd: &str) -> Result<VmCommand> {
    use Comparison as Cmp;
    use MemSegment as Seg;
    //asm.push(code_writer::comment(cmd)); // comment with original vm command, stored separately so it can be skipped
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let command = match parts.len() {
        1 => match parts[0] {
            "add" => VmCommand::Add,
            "sub" => VmCommand::Sub,
            "neg" => VmCommand::Neg,
            "eq" => VmCommand::Compare(Cmp::EQ),
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
