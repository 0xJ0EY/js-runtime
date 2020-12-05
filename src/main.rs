#[macro_use] extern crate lazy_static;

mod tokenizer;
mod ast;
mod runtime;
mod util;

fn main() {
    println!("Running Joey-Script 1.0");

    let input = String::from(include_str!("main.js"));
    let tokens = tokenizer::tokenize(&input);
    let ast = ast::parse(&tokens);

    runtime::run(&ast);
}
