use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::tokens::jack_tokens::{Keyword::*, Token};
use crate::tokens::vm_commands::{VmCommand, Comparison, MemSegment};
use super::CodeWriter;

#[derive(Default)]
pub struct VmWriter {
    writer: Option<BufWriter<File>>,
    if_counter: u16,
    while_counter: u16,
}

impl CodeWriter for VmWriter {
    fn new(filename: &str) -> Self {
        let file =
            File::create(Path::new(filename).with_extension("vm")).expect("could not create file");
        let writer = BufWriter::new(file);
        VmWriter {
            writer: Some(writer),
            if_counter: 0,
            while_counter: 0,
        }
    }

    fn write(&mut self, contents: impl Display) {
        writeln!(self.writer.as_mut().expect("no writer"), "{contents}").expect("failed to write");
    }

    fn flush(&mut self) {
        self.writer.as_mut().expect("no writer").flush().unwrap();
    }
}

impl VmWriter {
    pub fn generate_label(&mut self, label: &str) -> String {
        let counter = if label == "if" {
            &mut self.if_counter
        } else {
            &mut self.while_counter
        };
        let label = format!("{label}{counter}");
        *counter += 1;
        label
    }

    pub fn write_constant(&mut self, t: Token) {
        match t {
            Token::Keyword(True) => {
                self.write(VmCommand::Push(MemSegment::Constant, 1));
                self.write(VmCommand::Neg);
            }
            Token::Keyword(False) | Token::Keyword(Null) => {
                self.write(VmCommand::Push(MemSegment::Constant, 0))
            }
            Token::Keyword(This) => self.write(VmCommand::Push(MemSegment::Pointer, 0)),
            Token::IntConstant(i) => self.write(VmCommand::Push(MemSegment::Constant, i)),
            Token::StringConstant(s) => {
                self.write(VmCommand::Push(MemSegment::Constant, s.len() as i16));
                self.write(VmCommand::Call("String.new", 1));
                for c in s.chars() {
                    self.write(VmCommand::Push(MemSegment::Constant, c as i16));
                    self.write(VmCommand::Call("String.appendChar", 2));
                }
            }
            _ => { /*only passing constants*/ }
        }
    }
}
