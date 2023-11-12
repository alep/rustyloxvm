use std::u8;

pub enum OpCode {
    OpConstant = 0,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDevide,
    OpNegate,
    OpReturn,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpConstant,
            1 => OpCode::OpAdd,
            2 => OpCode::OpSubtract,
            3 => OpCode::OpMultiply,
            4 => OpCode::OpDevide,
            5 => OpCode::OpNegate,
            6 => OpCode::OpReturn,
            _ => unimplemented!("unimplemented opcode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}
// Chunks
//

pub struct Chunk {
    pub code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn read(&self, ip: usize) -> u8 {
        self.code[ip]
    }

    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let instruction: OpCode = self.code[offset].into();
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!(" |   ");
        } else {
            print!("{:4} ", self.lines[offset])
        }
        match instruction {
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset, &"\n"),
            OpCode::OpConstant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::OpAdd => self.simple_instruction("OP_ADD", offset, &""),
            OpCode::OpSubtract => self.simple_instruction("OP_SUBTRACT", offset, &""),
            OpCode::OpMultiply => self.simple_instruction("OP_MULTIPLY", offset, &""),
            OpCode::OpDevide => self.simple_instruction("OP_DEVIDED", offset, &""),
            OpCode::OpNegate => self.simple_instruction("OP_NEGATE", offset, &""),
        }
    }
    fn simple_instruction(&self, name: &str, offset: usize, end: &str) -> usize {
        println!("{}{}", name, end);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:-16} {:4} '", name, constant);
        self.print_value(self.get_constant(constant as usize));
        print!("'\n");
        offset + 2
    }

    pub fn add_constants(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.count() - 1
    }

    pub fn print_value(&self, value: &Value) {
        print!("{}", *value)
    }

    pub fn get_constant(&self, index: usize) -> &Value {
        self.constants.read_value(index)
    }
}

// Values
//

pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn read_value(&self, index: usize) -> &Value {
        &self.values[index]
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }
}

mod test_chunks {
    use crate::common::{Chunk, OpCode};

    #[test]
    fn test_chunks() {
        let mut chunk = Chunk::new();
        let constant = chunk.add_constants(1.2);
        chunk.write(OpCode::OpConstant.into(), 123);
        chunk.write(constant as u8, 123);
        chunk.write(OpCode::OpReturn.into(), 123);
        chunk.disassemble_chunk("test")
    }
}
