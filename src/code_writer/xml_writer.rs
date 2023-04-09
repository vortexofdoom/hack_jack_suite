use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::code_writer::CodeWriter;
use crate::tokens::jack_tokens::Token;

struct XMLWrapper {
    inner: Token,
}

impl Display for XMLWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Token::Keyword(k) => write!(f, "<keyword> {k} </keyword>"),
            Token::Identifier(s) => write!(f, "<identifier> {s} </identifier>"),
            Token::StringConstant(s) => write!(f, "<stringConstant> {s} </stringConstant>"),
            Token::IntConstant(i) => write!(f, "<integerConstant> {i} </integerConstant>"),
            Token::Symbol(c) => match c {
                '<' => write!(f, "<symbol> &lt; </symbol>"),
                '>' => write!(f, "<symbol> &gt; </symbol>"),
                '"' => write!(f, "<symbol> &quot; </symbol>"),
                '&' => write!(f, "<symbol> &amp; </symbol>"),
                _ => write!(f, "<symbol> {c} </symbol>"),
            },
        }
    }
}

#[derive(Default)]
pub struct XMLWriter {
    writer: Option<BufWriter<File>>,
}
impl CodeWriter for XMLWriter {
    fn new(filename: &str) -> Self {
        let file =
            File::create(Path::new(filename).with_extension("xml")).expect("could not create file");
        let writer = BufWriter::new(file);
        XMLWriter {
            writer: Some(writer),
        }
    }

    fn write(&mut self, contents: impl Display) {
        writeln!(self.writer.as_mut().unwrap(), "{contents}").expect("failed to write");
        self.flush();
    }

    fn flush(&mut self) {
        self.writer.as_mut().unwrap().flush().unwrap();
    }
}

impl XMLWriter {}
