use compiler::{
    chunk::{Chunk, OpCode},
    values::Value,
};
use debug::disassemble_chunk;
use vm::VM;

mod compiler;
mod debug;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    let constant_idx = chunk.add_value(Value::Number(1.2));
    chunk.write_code(OpCode::OpConstant(constant_idx as u8), 1);

    chunk.write_code(OpCode::OpNegate, 8);

    chunk.write_code(OpCode::OpReturn, 8);

    disassemble_chunk(&chunk, "Test Chunk".to_string());

    let mut vm = VM::new(&chunk);
    vm.interpret();
}
