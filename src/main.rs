mod compiler;
mod lexer;
mod minimal_lang;
mod parser;

mod builder_dir {
    pub mod builder;
    pub mod return_code;
}

fn main() {
    minimal_lang::compile("main.min", true);
}
