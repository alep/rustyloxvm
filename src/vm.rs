use crate::common::{Chunk, OpCode, Value};

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new() -> Self {
        VM {
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.ip = 0;
        self.run(chunk)
    }

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let opcode: OpCode = chunk.read(self.ip).into();
        self.ip += 1;
        opcode
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        loop {
            chunk.disassemble_instruction(self.ip);
            match self.read_byte(chunk) {
                OpCode::OpAdd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                OpCode::OpSubtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                OpCode::OpMultiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                OpCode::OpDevide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                OpCode::OpNegate => {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(-val)
                }
                OpCode::OpConstant => {
                    let constant_offest = self.read_byte(chunk);
                    let value = chunk.get_constant(constant_offest as usize);
                    self.stack.push(*value);
                }
                OpCode::OpReturn => {
                    chunk.print_value(&self.stack.pop().unwrap());
                    return InterpretResult::InterpretOk;
                }
            }
        }
    }
}
