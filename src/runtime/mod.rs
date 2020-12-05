use core::panic;
use std::collections::{HashMap, LinkedList};

use crate::ast::{AstProgram, nodes::{AstNode, CallExpressionCallee, ExpressionStatement, ExpressionStatementExpression, Literal}};

pub struct FunctionCall {
    function_type: FunctionCallType,
    arguments: Vec<Literal>,
}

pub struct SystemCall {
    func: Box<dyn Fn(&HashMap<String, Literal>) -> Option<Literal>>,
}

impl SystemCall {
    fn new(func: impl Fn(&HashMap<String, Literal>) -> Option<Literal> + 'static) -> Self {
        Self { 
            func: Box::new(func)
        }
    }
}

pub enum FunctionCallType {
    SystemCall(SystemCall),
    RuntimeCall
}

pub struct Variable {
}

pub struct BlockScope {
    pub functions: HashMap<String, FunctionCall>,
    pub variables: HashMap<String, Literal>,
}

impl BlockScope {
    pub fn new() -> BlockScope {
        BlockScope {
            functions: HashMap::<String, FunctionCall>::new(),
            variables: HashMap::<String, Literal>::new(),
        }
    }

    pub fn new_root() -> BlockScope {
        let mut functions = HashMap::<String, FunctionCall>::new();

        functions.insert("log".to_string(), FunctionCall { 
            function_type: FunctionCallType::SystemCall(
                SystemCall::new(|x| { 
                    let output = x.get(&"output".to_string()).unwrap(); 

                    println!("{}", output.value);
                    
                    None
                })), 
                arguments: vec![Literal::from_str("output")]
            }
        );

        BlockScope {
            functions,
            variables: HashMap::<String, Literal>::new(),
        }
    }
}

pub struct Runtime {
    pub scopes: Vec<BlockScope>,
}

impl Runtime {
    pub fn new() -> Runtime {
        let root_scope = BlockScope::new_root();
        let scopes = vec![root_scope];

        Runtime {
            scopes,
        }
    }

    pub fn run(&mut self, program: &AstProgram) {

        for step in program.body.iter() {
            match step {
                AstNode::ExpressionStatement(statement) => {
                    parse_expression_statement(self, statement);
                },
                _ => { panic!("Unsupported step: {:?}", step) }
            }
        }
    }

    pub fn function(&self, function_name: &String) -> Option<&FunctionCall> {
        let latest_scope = self.scopes.len();

        for index in (0..latest_scope).rev() {
            let scope = self.scopes.get(index);

            if scope.is_none() {
                break;
            }

            let scope = scope.unwrap();
            let function = scope.functions.get(function_name);

            if function.is_some() {
                return function;
            }
        }

        None
    }

}

fn parse_expression_statement(runtime: &mut Runtime, statement: &ExpressionStatement) {
    let name = parse_expression_statement_name(&statement);
    let function = runtime.function(&name);

    if function.is_none() {
        return;
    }

    let function = function.unwrap();

    let args_values = parse_expression_statement_args(statement);
    let args = &function.arguments;
    
    if args.len() != args_values.len() {
        panic!("Amount of params do not match");
    }

    let mut args_map = HashMap::<String, Literal>::new();

    for index in 0..args.len() {
        let key = args.get(index).unwrap().value.clone();
        let value = args_values.get(index)
            .unwrap()
            .clone();

        args_map.insert(key, value);
    }

    match &function.function_type {
        FunctionCallType::SystemCall(syscall) => {
            (syscall.func)(&args_map);
        },
        _ => { panic!("Unsupported function type") }
    }
}

fn parse_expression_statement_args(statement: &ExpressionStatement) -> &Vec<Literal> {
    match &statement.expression {
        ExpressionStatementExpression::CallExpression(expression) => {
            &expression.arguments
        },
        _ => { panic!("Unsupported expression: {:?}", &statement.expression) }
    }
}

fn parse_expression_statement_name(statement: &ExpressionStatement) -> String {

    match &statement.expression {
        ExpressionStatementExpression::CallExpression(expression) => {
            match &expression.callee {
                CallExpressionCallee::Identifier(identifier) => {
                    identifier.name.to_string()
                }
                _ => { panic!("Unsupported expression callee: {:?}", &expression.callee) }
            }
        },
        _ => { panic!("Unsupported expression: {:?}", &statement.expression) }
    }

}

pub fn run(program: &AstProgram) {
    let mut runtime = Runtime::new();
    runtime.run(program);
}