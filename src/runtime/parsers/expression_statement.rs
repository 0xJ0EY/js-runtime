use std::collections::HashMap;

use crate::{ast::nodes::ExpressionStatement, util::is_identifier, ast::nodes::{CallExpressionCallee, Literal, VariableLiteral}, runtime::{nodes::FunctionCallType, Runtime}, util::is_number};

pub fn parse_expression_statement(runtime: &mut Runtime, statement: &ExpressionStatement) {
    let name = parse_expression_statement_name(&statement);
    runtime.new_scope();
    let function = runtime.function(&name);

    if function.is_none() {
        runtime.pop_scope();
        return;
    }

    // Create temporary scope
    let function = function.unwrap();

    let args_values = parse_expression_statement_args(statement);
    let args = &function.arguments;
    
    if args.len() != args_values.len() {
        panic!("Amount of params do not match");
    }

    let mut args_map = HashMap::<String, Literal>::new();

    for index in 0..args.len() {
        let key = args.get(index).unwrap().value.clone();
        let mut value = args_values.get(index)
            .unwrap()
            .clone();

        // Fetch actual value if it is an variable
        let first_char = value.raw_value.chars()
            .nth(0)
            .unwrap();

        if is_identifier(&first_char) {
            let variable_name = value.value.clone();
            let variable = runtime.variable(&variable_name);

            if variable.is_none() {
                panic!("{:?} is not defined", variable)
            }


            match variable.unwrap() {
                VariableLiteral::Literal(literal) => {
                    value = literal.clone();
                },
                _ => { println!("Unsupported variable type") }
            }

        }

        args_map.insert(key, value);
    }

    match &function.function_type {
        FunctionCallType::SystemCall(syscall) => {
            (syscall.func)(&args_map);
        },
        _ => { panic!("Unsupported function type") }
    }

    // Exit temporary scope
    runtime.pop_scope();
}

fn parse_expression_statement_args(statement: &ExpressionStatement) -> &Vec<Literal> {
    todo!("refactor");

    // match &statement.expression {
    //     ExpressionStatementExpression::CallExpression(expression) => {
    //         &expression.arguments
    //     },
    //     _ => { panic!("Unsupported expression: {:?}", &statement.expression) }
    // }
}

fn parse_expression_statement_name(statement: &ExpressionStatement) -> String {
    todo!("refactor");

    // match &statement.expression {
    //     ExpressionStatementExpression::CallExpression(expression) => {
    //         match &expression.callee {
    //             CallExpressionCallee::Identifier(identifier) => {
    //                 identifier.name.to_string()
    //             }
    //             _ => { panic!("Unsupported expression callee: {:?}", &expression.callee) }
    //         }
    //     },
    //     _ => { panic!("Unsupported expression: {:?}", &statement.expression) }
    // }
}
