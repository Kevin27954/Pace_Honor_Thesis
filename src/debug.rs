use crate::compiler::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("=== {} ===", name);

    for i in 0..chunk.code.len() {
        disaseemble_code(&chunk, i);
    }
}

pub fn disaseemble_code(chunk: &Chunk, offset: usize) {
    if chunk.code.len() <= offset {
        return;
    }

    print!("{:04} ", offset);

    if offset > 0 && chunk.line[offset] == chunk.line[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.line[offset]);
    }

    let code = &chunk.code[offset];
    match code {
        OpCode::OpConstant(loc) => {
            println!("{} {:10} {}", code, loc, chunk.values[*loc as usize]);
        }
        OpCode::OpDefineGlobal(loc) => {
            println!("{} {:10} {}", code, loc, chunk.values[*loc as usize]);
        }
        OpCode::OpGetGlobal(loc) => {
            println!("{} {:10} {}", code, loc, chunk.values[*loc as usize]);
        }
        OpCode::OpSetGlobal(loc) => {
            println!("{} {:10} {}", code, loc, chunk.values[*loc as usize]);
        }
        OpCode::OpGetLocal(loc) => {
            println!("{} {:10} ", code, loc);
        }
        OpCode::OpSetLocal(loc) => {
            println!("{} {:10} ", code, loc);
        }
        OpCode::OpJumpIfFalse(loc) => {
            println!("{} {:10} ", code, loc);
        }

        // Add individal codes here if you want to debug
        _ => {
            println!("{}", chunk.code[offset]);
        }
    }
}
