use std::thread::current;

use crate::tokenizer::{Token, TokenType};

use super::{nodes::AstNode, nodes::BlockStatement, nodes::FunctionDeclaration, parser::AstParser, nodes::Identifier};

/*
Block statement
*/
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

        // Function declaration
        let function_declaration = parse_function_declaration(parser);
        if function_declaration.is_some() {
            let value = AstNode::FunctionDeclaration {
                inner: function_declaration.unwrap()
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

/*
Function declaration
*/
pub fn parse_function_declaration(parser: &mut AstParser) -> Option<FunctionDeclaration> {

    if !is_start_function_declaration(parser) {
        return None;
    }

    let keyword = parser.consume().unwrap();
    let start = keyword.range.0;

    let name = parser.consume().unwrap();

    let identifier = Identifier {
        name: name.value.clone(),
        range: (name.range.0, name.range.1)
    };

    parser.step(); // Step over param open

    if has_function_paremeter(parser.token().unwrap()) {
        // Parse params

    }

    parser.step(); // Step over param close
    
    let body = parse_block_statement(parser);
    
    if body.is_none() {
        return None;
    }

    let body = body.unwrap();
    let end = body.range.1;

    Some(
        FunctionDeclaration {
            id: identifier,
            params: Vec::new(),
            body,
            range: (start, end)
        }
    )
}

pub fn is_start_function_declaration(parser: &AstParser) -> bool {
    let mut current_step = 0; 

    // Function keyword
    let keyword = parser.peek_steps(current_step);
    if keyword.is_none() || !is_function_keyword(keyword.unwrap()) {
        return false;
    }
    
    current_step += 1;

    // Function name
    let function_name = parser.peek_steps(current_step);
    if function_name.is_none() || !is_function_name(function_name.unwrap()) {
        return false;
    }
    
    current_step += 1;

    // Parenthesis
    let open_parenthesis = parser.peek_steps(current_step);
    if open_parenthesis.is_none() || !is_function_open_parenthesis(open_parenthesis.unwrap()) {
        return false;
    }

    current_step += 1;

    // TODO: Also check for params

    true
}

fn is_function_keyword(token: &Token) -> bool {
    token.token_type == TokenType::Identifier && token.value == String::from("function")
}

fn is_function_name(token: &Token) -> bool {
    token.token_type == TokenType::Identifier
}


fn is_function_open_parenthesis(token: &Token) -> bool {
    token.token_type == TokenType::Parenthesis && token.value == String::from("(")
}

fn has_function_paremeter(token: &Token) -> bool {
    false
}

fn is_function_close_parenthesis(token: &Token) -> bool {
    token.token_type == TokenType::Parenthesis && token.value == String::from(")")
}