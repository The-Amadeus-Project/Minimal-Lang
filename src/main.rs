use std::fs;
use std::fs::File;

mod compiler;
mod lexer;
mod minimal_lang;
mod parser;

mod builder_dir {
    pub mod builder_nasm;
    pub mod return_code;
    pub mod builder_gas;
}

fn main() {
    let out_file = "out.asm";
    let compiled = minimal_lang::compile("main.min", true);
    let _res = File::create(out_file);
    let _ = fs::write(out_file, compiled);
}
