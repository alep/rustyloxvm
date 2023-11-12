mod common;
mod errors;
mod vm;

fn main() {
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
