use crate::compiler::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("=== {} ===", name);

    for i in 0..chunk.code.len() {
        disaseemble_code(&chunk, i);
    }
}

pub fn disaseemble_code(chunk: &Chunk, offset: usize) {
    print!("{:04} ", offset);

    if offset > 0 && chunk.line[offset] == chunk.line[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.line[offset]);
    }

    let code = &chunk.code[offset];
    match code {
        OpCode::OpAdd => {
            println!("{}", chunk.code[offset]);
        }
        OpCode::OpConstant(loc) => {
            println!("{} {:10} {}", code, loc, chunk.values[*loc as usize]);
        }
        OpCode::OpDivide => {
            println!("{}", chunk.code[offset]);
        }
        OpCode::OpMultiply => {
            println!("{}", chunk.code[offset]);
        }
        OpCode::OpNegate => {
            println!("{}", chunk.code[offset]);
        }
        OpCode::OpReturn => {
            println!("{}", chunk.code[offset]);
        }
        OpCode::OpSubtract => {
            println!("{}", chunk.code[offset]);
        }
    }
}
