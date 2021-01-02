use crate::{ast::nodes::FunctionDeclaration, runtime::{Runtime, nodes::{FunctionCall, FunctionCallType}}};

pub fn parse_function_declaration(runtime: &mut Runtime, statement: &FunctionDeclaration) {
    let current_scope = runtime.current_scope();
    
    let name = statement.id.name.to_string();
    let params = statement.to_owned().params;
    let body = statement.to_owned().body;    

    let function_call = FunctionCall {
        function_type: FunctionCallType::RuntimeCall(body),
        arguments: vec![],
    };

    current_scope.functions.insert(name, function_call);    
}