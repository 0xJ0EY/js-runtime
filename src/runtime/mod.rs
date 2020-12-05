extern crate regex;
use regex::Regex;

use core::panic;
use std::collections::{HashMap, LinkedList};

use crate::ast::{AstProgram, nodes::{AstNode, CallExpressionCallee, ExpressionStatement, ExpressionStatementExpression, Literal, VariableDeclaration, VariableLiteral}};

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
    pub variables: HashMap<String, VariableLiteral>,
}

impl BlockScope {
    pub fn new() -> BlockScope {
        BlockScope {
            functions: HashMap::<String, FunctionCall>::new(),
            variables: HashMap::<String, VariableLiteral>::new(),
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
            variables: HashMap::<String, VariableLiteral>::new(),
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
                AstNode::VariableDeclaration(variable) => {
                    parse_variable_declaration(self, variable);
                },
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

    pub fn variable(&self, variable_name: &String) -> Option<&VariableLiteral> {
        let latest_scope = self.scopes.len();

        for index in (0..latest_scope).rev() {
            let scope = self.scopes.get(index);

            if scope.is_none() {
                break;
            }

            let scope = scope.unwrap();
            let variable = scope.variables.get(variable_name);

            if variable.is_some() {
                return variable;
            }
        }

        None
    }

    pub fn new_scope(&mut self) {
        self.scopes.push(BlockScope::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn current_scope(&mut self) -> &mut BlockScope {
        let index = self.scopes.len() - 1;
        self.scopes.get_mut(index).unwrap()
    }

}

fn parse_variable_declaration(runtime: &mut Runtime, declaration: &VariableDeclaration) {
    for variable in declaration.declarations.iter() {
        let name = variable.id.name.clone();
        let literal = variable.init.clone();

        runtime.current_scope().variables.insert(name, literal);
    } 

}

fn parse_expression_statement(runtime: &mut Runtime, statement: &ExpressionStatement) {
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

        if !is_number(&first_char) && first_char != '\'' && first_char != '\"' {
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


fn is_number(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
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