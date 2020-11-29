use crate::tokenizer::{Token, TokenType};

use super::{nodes::AstNode, nodes::BlockStatement, parser::AstParser, nodes::FunctionDeclaration};

pub fn parse_block_statement(parser: &mut AstParser) -> Option<BlockStatement> {

    if !is_start_block_statement(parser.token().unwrap()) {
        return None;
    }

    let mut body = Vec::new(); 

    // Always skip the first bracket by stepping over it
    let mut consumed = parser.consume();
    let start = consumed.unwrap().range.0;

    while consumed.is_some() && !is_end_block_statement(&consumed.unwrap()) {

        // Block statement
        let block_statement = parse_block_statement(parser);
        if block_statement.is_some() {
            let value = AstNode::BlockStatement {
                inner: block_statement.unwrap()
            };

            body.push(value);
        }

        consumed = parser.consume();
    }

    let end = consumed.unwrap().range.1;

    Some(BlockStatement {
        body: body,
        range: (start, end)
    })
}

pub fn is_start_block_statement(token: &Token) -> bool {
    token.token_type == TokenType::CurlyBraces && token.value == String::from("{")
}

pub fn is_end_block_statement(token: &Token) -> bool {
    token.token_type == TokenType::CurlyBraces && token.value == String::from("}")
}

pub fn parse_function_statement(parser: &mut AstParser) -> Option<FunctionDeclaration> {
    None
}