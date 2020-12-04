use std::collections::LinkedList;

use crate::ast::AstProgram;




pub enum FunctionCall {
    SystemCall,
    RuntimeCall,
}

pub struct Variable {
}


pub struct BlockScope {
    pub functions: HashMap<Identity, FunctionCall>,
    pub variables: Vec<Variable>
}

pub fn run(program: &AstProgram) {

    let mut scopes = LinkedList::<BlockScope>::new();
    
    let functions = Vec::from([
        FunctionCall::SystemCall,
    ]);

    scopes.push_back(
        BlockScope {
            functions: functions,
            variables: Vec::new(),
        }
    )

    
}