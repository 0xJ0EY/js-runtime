
use core::panic;
use crate::{ast::{AstProgram, nodes::{AstNode, VariableLiteral}}};

use self::{nodes::{BlockScope, FunctionCall}, parsers::{expression_statement::parse_expression_statement, variable_declaration::parse_variable_declaration}};

mod nodes;
mod parsers;

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
                AstNode::FunctionDeclaration(declaration) => {
                    panic!("panik");  
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

pub fn run(program: &AstProgram) {
    let mut runtime = Runtime::new();
    runtime.run(program);
}