use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::vec;

use crate::code_writer::asm_writer::AsmWriter;
pub mod parser;

fn translate_vm(bootstrap: bool) {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<PathBuf> = vec![];
    let file_path = Path::new(&args[1]);
    let filename = file_path.file_stem().unwrap().to_str().unwrap();
    if file_path.is_dir() {
        for entry in file_path.read_dir().unwrap() {
            if let Some(x) = entry.as_ref().unwrap().path().extension() {
                if x.to_str().unwrap() == "vm" {
                    files.push(entry.as_ref().unwrap().path())
                }
            }
        }
    } else if let Some("vm") = file_path.extension().unwrap().to_str() {
        files.push(file_path.to_path_buf())
    }
    let mut writer = AsmWriter::new(filename, bootstrap);
    for file in files {
        if let Ok(f) = File::open(&file) {
            writer.set_file_name(file.file_stem().unwrap().to_str().unwrap());
            let reader = BufReader::new(f);
            for line in reader.lines().flatten() {
                let cmd = line
                    .find("//")
                    .map(|i| &line[..i])
                    .unwrap_or(&line)
                    .trim()
                    .to_string();
                if !cmd.is_empty() {
                    let vm_cmd = parser::parse(&cmd).expect("could not parse command");
                    writer.generate_code(vm_cmd, true);
                }
            }
        }
    }
    writer.flush();
}
