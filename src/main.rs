#[allow(dead_code)]
mod code_writer;
mod cpu;
mod jack_compiler;
mod tokens;
mod vm_translator;

use crate::jack_compiler::compilation_engine::CompilationEngine;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<PathBuf> = vec![];
    let file_path = Path::new(&args[1]);
    let mut parser = CompilationEngine::new();
    if file_path.is_dir() {
        for entry in file_path.read_dir().unwrap() {
            if let Some(x) = entry.as_ref().unwrap().path().extension() {
                if x.to_str().unwrap() == "jack" {
                    files.push(entry.as_ref().unwrap().path())
                }
            }
        }
    } else if let Some("jack") = file_path.extension().unwrap().to_str() {
        files.push(file_path.to_path_buf())
    }
    for file in files {
        parser.compile(file).expect("error");
    }
}
