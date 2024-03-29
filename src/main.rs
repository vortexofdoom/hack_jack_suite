#![allow(dead_code)]
#![allow(clippy::unusual_byte_groupings)]
mod optimizer;
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
use io::{get_key, SCREEN_ROW_BYTES};
use sdl2::{event::Event, pixels::Color};
use std::path::PathBuf;

use crate::io::SCREEN_PIXELS;

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
        .window("Hack Emulator", 1024, 512)
        .position_centered()
        .build()
        .expect("Could not initialize window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    canvas
        .set_logical_size(512, 256)
        .expect("Could not set logical size");

    let creator = canvas.texture_creator();

    let mut screen = creator
        .create_texture_streaming(Some(sdl2::pixels::PixelFormatEnum::RGB24), 512, 256)
        .expect("failed to create texture");
    screen.update(None, &[255; SCREEN_PIXELS], SCREEN_ROW_BYTES).unwrap();
    //let screen = Screen::new(canvas, texture);

    let mut assembler = Assembler::new();

    let asm = assembler.assemble(&asm_macro::asm![
    ("START")
        @KBD
        D=M
        @"PRESS"
        D;JNE
        @"NO_PRESS"
        D;JEQ

    // fill = is_pressed ? -1 : 0
    ("PRESS")
        @"fill"
        M=-1
        @"CHECK"
        0;JMP
    ("NO_PRESS")
        @"fill"
        M=0

    // Only update screen if fill has changed since last fill
    ("CHECK")
        @"last"
        D=M
        @"fill"
        D=D-M
        @"START"
        D;JEQ

    // Initialize fill parameters
        @8192
        D=A
        @"i"
        M=D // i = 8192
        @SCREEN
        D=A
        @"address"
        M=D // address = start of screen RAM
        @"fill"
        D=M
        @"last"
        M=D     // filling, so set last fill to current

    ("FILL_LOOP")
        @"i"
        D=M
        @"START"
        D;JLE // while i > 0

        @"fill"
        D=M
        @"address"
        A=M
        M=D // addr = fill

        @"i"
        M=M-1 // i--
        @"address"
        M=M+1 // addr++
        @"FILL_LOOP"
        0;JMP

    ]);

    
    let mut cpu = Cpu::new(&asm);
    let mut event_pump = sdl_context.event_pump()?;
    let mut last_frame = std::time::Instant::now();
    let mut ticks = 0;
    let start = last_frame;
    let mut frames = 0;
    'running: loop {
        if last_frame.elapsed().as_millis() >= 50 {
            canvas.copy(&screen, None, None)?;
            canvas.present();
            last_frame = std::time::Instant::now();
            while let Some(event) = event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { .. } | Event::KeyUp { .. } => {
                        cpu.set_kbd(get_key(event_pump.keyboard_state()))
                    }
                    _ => {}
                }
            }
            frames += 1;
        }
        if let Some(update) = cpu.tick().map_err(|e| e.to_string())? {
            screen
                .update(update.rect, &update.pixels, SCREEN_ROW_BYTES)
                .map_err(|e| e.to_string())?;
        }
        ticks += 1;
    }
    let elapsed = start.elapsed().as_secs_f64();
    println!("{}", ticks as f64 / elapsed);
    println!("{}", ticks as f64 / frames as f64);

    Ok(())
}
