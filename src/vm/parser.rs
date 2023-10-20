use anyhow::{anyhow, Result};

use super::{
    Comparison::{Eq, GT, LT},
    MemSegment as Seg, VmCommand,
};

pub fn parse(cmd: &str) -> Result<VmCommand> {
    //asm.push(code_writer::comment(cmd)); // comment with original vm command, stored separately so it can be skipped
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let command = match parts.len() {
        1 => match parts[0] {
            "add" => VmCommand::Add,
            "sub" => VmCommand::Sub,
            "neg" => VmCommand::Neg,
            "eq" => VmCommand::Compare(Eq),
            "gt" => VmCommand::Compare(GT),
            "lt" => VmCommand::Compare(LT),
            "and" => VmCommand::And,
            "or" => VmCommand::Or,
            "not" => VmCommand::Not,
            "return" => VmCommand::Return,
            _ => return Err(anyhow!("No one word command \"{cmd}\"")),
        },
        2 => match parts[0] {
            "label" => VmCommand::Label(parts[1]),
            "goto" => VmCommand::Goto(parts[1]),
            "if-goto" => VmCommand::IfGoto(parts[1]),
            _ => return Err(anyhow!("No two word command \"{cmd}\"")),
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

                _ => return Err(anyhow!("No three word command \"{cmd}\"")),
            }
        }
        _ => return Err(anyhow!("\"{cmd}\" is not a valid VM command")),
    };
    Ok(command)
}
