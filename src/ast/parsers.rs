use std::{cell::RefCell, collections::VecDeque};

use crate::tokenizer::{Token, TokenType};

use super::{nodes::AstNode, nodes::BlockStatement, nodes::CallExpression, nodes::ExpressionStatement, nodes::FunctionDeclaration, nodes::Identifier, nodes::Literal, nodes::VariableDeclaration, nodes::VariableDeclarator, nodes::{CallExpressionCallee, MemberExpression, VariableLiteral}, parser::AstParser};

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
            let value = AstNode::BlockStatement(block_statement.unwrap());
            body.push(value);
        }

        // Function declaration
        let function_declaration = parse_function_declaration(parser);
        if function_declaration.is_some() {
            let value = AstNode::FunctionDeclaration(function_declaration.unwrap());

            body.push(value);
        }

        // Variable declaration
        let variable_declaration = parse_variable_declaration(parser);
        if variable_declaration.is_some() {
            let value = AstNode::VariableDeclaration(variable_declaration.unwrap());

            body.push(value);
        }

        // Expression statement
        let expression_statement = parse_expression_statement(parser);
        if expression_statement.is_some() {
            let value = AstNode::ExpressionStatement(expression_statement.unwrap());

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

/*
Variable declaration
*/
pub fn parse_variable_declaration(parser: &mut AstParser) -> Option<VariableDeclaration> {
    let mut declarations = Vec::<VariableDeclarator>::new();

    if !is_variable_declaration(parser) {
        return None;
    }

    let keyword = parser.consume().unwrap();
    let start = keyword.range.0;

    let name = parser.consume().unwrap();
    let name_range = name.range.clone();

    let identifier = Identifier {
        name: name.value.clone(),
        range: (name.range.0, name.range.1)
    };

    parser.step(); // Skip assignment

    let value = parser.consume().unwrap();
    let value_range = value.range.clone();

    let literal = match value.token_type {
        TokenType::Number |
        TokenType::String => { 
            Some(VariableLiteral::Literal(Literal{
                value: value.value.clone(),
                raw_value: value.raw_value.clone(),
                range: value.range, 
            }))
        },
        _ => { None }
    };
    
    let literal = literal.unwrap();

    let declarator = VariableDeclarator {
        id: identifier,
        init: literal,
        range: (name_range.0, value_range.1)
    };

    declarations.push(declarator);

    let terminator = parser.consume().unwrap();
    let end = terminator.range.1;

    Some(VariableDeclaration {
        declarations: declarations,
        range: (start, end)
    })
}

// TODO: Currently we only support single variables
pub fn is_variable_declaration(parser: &AstParser) -> bool {

    let keyword = parser.peek_steps(0);
    if keyword.is_none() || !is_variable_keyword(keyword.unwrap()) {
        return false;
    }

    let name  = parser.peek_steps(1);
    if name.is_none() || !is_variable_name(name.unwrap()) {
        return false;
    }

    let assignment_operator = parser.peek_steps(2);
    if assignment_operator.is_none() || !is_variable_assignment(assignment_operator.unwrap()) {
        return false;
    }

    let init =  parser.peek_steps(3);
    if init.is_none() {
        return false;
    }

    let terminator = parser.peek_steps(4);
    if terminator.is_none() || !is_variable_terminator(terminator.unwrap()) {
        return false;
    }

    true
}

fn is_variable_keyword(token: &Token) -> bool {
    token.token_type == TokenType::Identifier && token.value == String::from("var")
}

fn is_variable_name(token: &Token) -> bool {
    token.token_type == TokenType::Identifier
}

fn is_variable_assignment(token: &Token) -> bool {
    token.token_type == TokenType::Operator && token.value == String::from("=")
}

fn is_variable_terminator(token: &Token) -> bool {
    token.token_type == TokenType::Terminator && token.value == String::from(";")
}

/*
Expression statement
*/
pub fn parse_expression_statement(parser: &mut AstParser) -> Option<ExpressionStatement> {

    let mut identifiers = VecDeque::new();
    let mut arguments = Vec::new();

    let start = parser.peek_current()
        .unwrap()
        .range
        .clone().0;

    loop {
        let name = parser.consume().unwrap();

        let identifier = Box::new(Identifier {
            name: name.value.clone(),
            range: name.range.clone()
        });

        identifiers.push_back(identifier);

        let seperator = parser.peek_current().unwrap();

        if !is_expression_seperator(seperator) {
            break;
        }

        parser.step();
    }

    parser.step(); // Skip open paren

    if parser.token().is_some() && is_expression_param(parser.token().unwrap()) {
        
        loop {
            let param = parser.consume();
            if param.is_some() {
                let param = param.unwrap();

                arguments.push(
                    Literal {
                        raw_value: param.raw_value.clone(),
                        value: param.value.clone(),
                        range: param.range,
                    }
                );
            }

            let seperator = parser.token();

            if seperator.is_none() {
                break;
            }

            if seperator.is_some() && is_expression_param_seperator(seperator.unwrap()) {
                parser.step();
            } else {
                break;
            }
        }

    }

    parser.step(); // Skip close paren
    
    let terminator = parser.consume().unwrap();
    let end = terminator.range.1;

    Some(ExpressionStatement {
        expression: CallExpression {
            callee: build_callee(&mut identifiers),
            arguments,
            range: (start, end)
        },
        range: (start, end)
    })
}

fn build_callee(identifiers: &mut VecDeque<Box<Identifier>>) -> CallExpressionCallee {

    // If the identifiers length == 1, return an CallExpression
    // Otherwise build a stack from the MemberExpressions
    if identifiers.len() == 1 {
        let identifier = identifiers.remove(0).unwrap();
        return CallExpressionCallee::Identifier(identifier);

    } else {
        let identifier = identifiers.pop_front().unwrap();

        // Use RefCell for dynamic borrow checking, as the while loop doesn't really like the static borrow checker from the compiler.
        let root_member_expression = RefCell::new(MemberExpression::new(*identifier));

        // Use this reference of a RefCell to keep track of the lastest item in the stack.
        let last_expression_ref = &root_member_expression;

        while identifiers.len() > 0 {
            let identifier = identifiers.pop_front().unwrap();

            let member_expression = RefCell::new(MemberExpression::new(*identifier));

            last_expression_ref.swap(&member_expression);

            // Wrap the value in a box, to store it on the heap
            let member_expression_object = Some(
                Box::new(
                    member_expression.borrow_mut().to_owned()
                )
            );

            last_expression_ref.try_borrow_mut()
                .unwrap()
                .object = member_expression_object;
        }

        let boxed_root_member_expression = Box::new(
            root_member_expression.borrow_mut().to_owned()
        );
        return CallExpressionCallee::MemberExpression(boxed_root_member_expression);
    }
}

pub fn is_expression_statement(parser: &AstParser) -> bool {

    // Check if the expression statement has MemberExpressions
    // Defined by a chain of 'Literal' -> '.' until the final expression
    // For example: console.log('foobar');
    // Where console is the MemberExpression and log is the last expression (So the call expression)

    let mut offset = 0;
    
    loop {
        let name = parser.peek_steps(offset);
        if name.is_none() || !is_expression_name(name.unwrap()) {
            break;
        }

        offset += 1;

        let seperator = parser.peek_steps(offset);
        if seperator.is_none() || !is_expression_seperator(seperator.unwrap()) {
            break;
        }

        offset += 1;
    }

    let open_paren = parser.peek_steps(offset);
    if open_paren.is_none() || !is_function_open_parenthesis(open_paren.unwrap()) {
        return false;
    }

    // Check for parameters
    let param = parser.peek_steps(offset + 1);

    if param.is_some() && is_expression_param(param.unwrap()) {
        loop {
            let param = parser.peek_steps(offset + 1);
    
            if param.is_none() {
                return false;
            }

            if param.is_some() && is_function_close_parenthesis(param.unwrap()) {
                break;
            }

            offset += 1;
        }
    }

    let close_paren = parser.peek_steps(offset + 1);
    if close_paren.is_none() || !is_function_close_parenthesis(close_paren.unwrap()) {
        return false;
    }

    let terminator = parser.peek_steps(offset + 2);
    if terminator.is_none() || !is_variable_terminator(terminator.unwrap()) {
        return false;
    }

    true
}

fn is_expression_param(token: &Token) -> bool {
    token.token_type == TokenType::Identifier || 
    token.token_type == TokenType::Number || 
    token.token_type == TokenType::String
}

fn is_expression_seperator(token: &Token) -> bool {
    token.token_type == TokenType::Separator && token.value == ".".to_string()
}

fn is_expression_param_seperator(token: &Token) -> bool {
    token.token_type == TokenType::Separator && token.value == ",".to_string()
}

fn is_expression_name(token: &Token) -> bool {
    token.token_type == TokenType::Identifier
}