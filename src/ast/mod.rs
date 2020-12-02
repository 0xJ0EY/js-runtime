use crate::tokenizer::Token;

use self::{nodes::AstNode};

mod parser;
mod parsers;
mod nodes;

#[derive(Debug)]
pub struct AstProgram {
    body: Vec<AstNode>,
}

impl AstProgram {
    pub fn new() -> AstProgram {
        AstProgram {
            body: Vec::new(),
        }
    }
}

pub fn parse(tokens: &Vec<Token>) -> Option<AstProgram> {
    parser::parse(tokens);

    None
}