use core::panic;

use crate::tokenizer::Token;

use super::{AstProgram, nodes::AstNode, parsers::{is_expression_statement, is_start_block_statement, is_start_function_declaration, is_variable_declaration, parse_block_statement, parse_expression_statement, parse_function_declaration, parse_variable_declaration}};

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

pub fn parse(tokens: &Vec<Token>) -> AstProgram {
    let mut program = AstProgram::new();
    let mut parser = AstParser::new(tokens);

    while parser.has_tokens() {

        let token = parser.token().unwrap();

        if is_start_block_statement(token) {
            let block_statement = parse_block_statement(&mut parser);
            let value = AstNode::BlockStatement(block_statement.unwrap());
            program.body.push(value);

            continue;
        }

        if is_start_function_declaration(&parser) {
            let function_declaration = parse_function_declaration(&mut parser);
            let value = AstNode::FunctionDeclaration(function_declaration.unwrap());
            program.body.push(value);

            continue;
        }

        if is_variable_declaration(&parser) {
            let variable_declaration = parse_variable_declaration(&mut parser);
            let value = AstNode::VariableDeclaration(variable_declaration.unwrap());
            program.body.push(value);
            
            continue;
        }

        if is_expression_statement(&parser) {
            let expression_statement = parse_expression_statement(&mut parser);
            let value = AstNode::ExpressionStatement(expression_statement.unwrap());
            program.body.push(value);

            continue;
        }

        panic!("Unknown token: {:?}", parser.token().unwrap().clone());
    }

    program
}