#![allow(dead_code)]
#![allow(clippy::unusual_byte_groupings)]

mod asm;
mod cpu;
mod vm;
//mod code_writer;
mod io;
//mod jack_compiler;
//mod tokens;

use asm::Assembler;
use anyhow::Result;
//use crate::jack_compiler::compilation_engine::CompilationEngine;
use clap::{Args, Parser, Subcommand};
use cpu::Cpu;
use io::Screen;
use sdl2::event::{self, Event};
use std::path::{Path, PathBuf};

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

fn main() -> Result<(), String> {

    // let args: Vec<String> = std::env::args().collect();
    // let mut files: Vec<PathBuf> = vec![];
    // let file_path = Path::new(&args[1]);
    // //let mut parser = CompilationEngine::new();
    // if file_path.is_dir() {
    //     for entry in file_path.read_dir().unwrap() {
    //         if let Some(x) = entry.as_ref().unwrap().path().extension() {
    //             if x.to_str().unwrap() == "jack" {
    //                 files.push(entry.as_ref().unwrap().path())
    //             }
    //         }
    //     }
    // } else if let Some("jack") = file_path.extension().unwrap().to_str() {
    //     files.push(file_path.to_path_buf())
    // }
    // for file in files {
    //     parser.compile(file).expect("error");
    // }

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let window = video_subsys.window("Hack Emulator", 512, 256)
        .position_centered()
        .build()
        .expect("Could not initialize window");

    let canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    let creator = canvas.texture_creator();

    let texture = creator.create_texture_streaming(Some(sdl2::pixels::PixelFormatEnum::RGB24), 512, 256).expect("failed to create texture");

    let screen = Screen::new(canvas, texture);
    let mut cpu = Cpu::new(screen);

    let mut event_pump = sdl_context.event_pump()?;
    
    let mut assembler = Assembler::new();

    let asm = assembler.assemble(&asm_macro::asm![
        @0
        D=M
        @23
        D;JLE
        @16
        M=D
        @16384
        D=A
        @17
        M=D
        @17
        A=M
        M=-1
        @17
        D=M
        @32
        D=D+A
        @17
        M=D
        @16
        MD=M-1
        @10
        D;JGT
        @23
        0;JMP
    ]);

    cpu.execute_asm(&asm)
        .map_err(|e| e.to_string())?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }


    Ok(())
}
