mod code_writer;
mod cpu;
mod jack_compiler;
mod tokens;
mod vm_translator;

use crate::jack_compiler::compilation_engine::CompilationEngine;
use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Parser)]
pub struct ProgArgs {
    #[clap(subcommand)]
    pub sub_command: HackArgs,
}

#[derive(Debug, Subcommand)]
pub enum HackArgs {
    Compile(CompileArgs),
}

#[derive(Debug, Args)]
pub struct CompileArgs {
    /// Path to the file to be compiled
    pub path: PathBuf,
    pub vm: bool,
}

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
