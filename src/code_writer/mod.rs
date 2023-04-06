pub(crate) mod asm_writer;
pub(crate) mod assembler;
pub(crate) mod vm_writer;
pub(crate) mod xml_writer;


pub trait CodeWriter: Default {
    fn write(&mut self, contents: impl std::fmt::Display);
    fn flush(&mut self);
    fn new(filename: &str) -> Self;
}