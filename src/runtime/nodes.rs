use std::collections::HashMap;

use crate::ast::nodes::{Literal, VariableLiteral};

pub struct FunctionCall {
    pub function_type: FunctionCallType,
    pub arguments: Vec<Literal>,
}

pub struct SystemCall {
    pub func: Box<dyn Fn(&HashMap<String, Literal>) -> Option<Literal>>,
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

        functions.insert("console.log".to_string(), FunctionCall { 
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