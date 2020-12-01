use core::panic;

use crate::tokenizer::Token;

use super::{nodes::AstNode, AstProgram, parsers::{parse_block_statement, parse_function_declaration}};

pub struct AstParser<'a> {
    index: usize,
    tokens: &'a Vec<Token>,
}

impl<'a> AstParser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> AstParser {
        AstParser {
            index: 0,
            tokens
        }
    }

    pub fn has_tokens(&self) -> bool {
        self.token().is_some()
    }

    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn peek_steps(&self, steps: usize) -> Option<&Token> {
        self.tokens.get(self.index + steps)
    }

    pub fn peek_back(&self, steps: usize) -> Option<&Token> {
        self.tokens.get(self.index - steps)
    }

    pub fn step(&mut self) {
        self.index += 1;
    }

    pub fn step_back(&mut self) {
        self.index -= 1;
    }

    pub fn consume(&mut self) -> Option<&Token> {
        let value = self.tokens.get(self.index);
        
        self.index += 1;

        return value;
    }
}

pub fn parse(tokens: &Vec<Token>) -> Option<()> {
    let mut program = AstProgram::new();
    let mut parser = AstParser::new(tokens);

    while parser.has_tokens() {

        let block_statement = parse_block_statement(&mut parser);
        if block_statement.is_some() {
            let value = AstNode::BlockStatement { inner: block_statement.unwrap() };
            program.body.push(value);

            continue;
        }

        let function_declaration = parse_function_declaration(&mut parser);
        if function_declaration.is_some() {
            let value = AstNode::FunctionDeclaration { inner: function_declaration.unwrap() };
            program.body.push(value);

            continue;
        }

        panic!("Unknown token: {:?}", parser.token().unwrap().clone());
    }

    dbg!(program);

    Some(())
}