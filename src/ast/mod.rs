use crate::tokenizer::Token;

use self::{nodes::AstNode};

mod parser;
mod parsers;
pub mod nodes;

#[derive(Debug)]
pub struct AstProgram {
    pub body: Vec<AstNode>,
}

impl AstProgram {
    pub fn new() -> AstProgram {
        Self {
            body: Vec::new(),
        }
    }

    pub fn from_body(body: &Vec<AstNode>) -> AstProgram {
        Self {
            body: body.clone(),
        }
    }
}

pub fn parse(tokens: &Vec<Token>) -> AstProgram {
    parser::parse(tokens)
}