use std::{io, io::prelude::*};

use std::process::exit;

mod common;
mod compiler;
mod errors;
mod scanner;
mod vm;

#[derive(Debug)]
struct IOError;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");
    return line.trim().to_string();
}

fn run_script(instance: &vm::VM, script_path: &String) -> Result<(), IOError> {
    let file = match std::fs::File::open(script_path) {
        Ok(k) => k,
        Err(e) => {
            return Err(IOError);
        }
    };
    let mut buf_reader = io::BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => {
            return Err(IOError);
        }
    }
    // self.execute_code(&mut contents)?;
    Ok(())
}

fn run_repl() {
    todo!()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let instance = vm::VM::new();

    match &args[..] {
        [_, script_path] => {
            match run_script(&instance, script_path) {
                Ok(_) => {}
                Err(err) => print!("Error: {:?}", err),
            };
        }
        [_] => {
            todo!()
        }
        [program_name, ..] => {
            exit(64);
        }
        _ => unreachable!("Unreacheable"),
    }

    let mut chunk = common::Chunk::new();
    let a = chunk.add_constants(1.2);
    chunk.write(common::OpCode::OpConstant as u8, 1);
    chunk.write(a as u8, 1);
    let b = chunk.add_constants(3.4);
    chunk.write(common::OpCode::OpConstant as u8, 1);
    chunk.write(b as u8, 1);
    chunk.write(common::OpCode::OpAdd as u8, 1);
    let c = chunk.add_constants(5.6);
    chunk.write(common::OpCode::OpConstant as u8, 1);
    chunk.write(c as u8, 1);
    chunk.write(common::OpCode::OpDevide as u8, 1);
    chunk.write(common::OpCode::OpReturn as u8, 1);
    // chunk.disassemble_chunk("test");
    let mut vm_instance = vm::VM::new();
    vm_instance.interpret(&chunk);
}
