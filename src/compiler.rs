use crate::scanner::Scanner;

pub mod chunk;
pub mod common;
pub mod values;

fn compiler(source: String) {
    let scanner = Scanner::new(source);
    let mut line = -1;
    loop {
        break;
    }
}

fn init_scanner(source: String) {
    unimplemented!();
}
