#![allow(dead_code)]
#![allow(clippy::unusual_byte_groupings)]

mod asm;
mod cpu;
mod vm;
//mod code_writer;
mod io;
//mod jack_compiler;
//mod tokens;

use anyhow::Result;
use asm::Assembler;
//use crate::jack_compiler::compilation_engine::CompilationEngine;
use clap::{Args, Parser, Subcommand};
use cpu::Cpu;
use io::{get_key, Screen};
use sdl2::event::Event;
use std::path::PathBuf;

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

    let window = video_subsys
        .window("Hack Emulator", 512, 256)
        .position_centered()
        .build()
        .expect("Could not initialize window");

    let canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    let creator = canvas.texture_creator();

    let texture = creator
        .create_texture_streaming(Some(sdl2::pixels::PixelFormatEnum::RGB24), 512, 256)
        .expect("failed to create texture");

    let screen = Screen::new(canvas, texture);
    let mut cpu = Cpu::new(screen);

    let mut event_pump = sdl_context.event_pump()?;

    let mut assembler = Assembler::new();

    let i = "i";
    let address = "address";
    const LOOP: &str = "LOOP";
    const END: &str = "END";
    let asm = assembler.assemble(&asm_macro::asm![
        @65
        D=A
        @R0
        M=D
        D=M
        @i
        M=D

        @SCREEN
        D=A
        @address
        M=D

    ("LOOP")
        @i
        D=M
        @END
        D;JEQ

        @address
        A=M
        M=-1

        @i
        M=M-1
        @32
        D=A
        @address
        M=D+M
        @LOOP
        0;JMP

    ("END")
        @END
        0;JMP
    ]);

    'running: loop {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    cpu.set_kbd(get_key(event_pump.keyboard_state()))
                }
                _ => {}
            }
        }
        cpu.tick(&asm).map_err(|e| e.to_string())?;
    }

    Ok(())
}
