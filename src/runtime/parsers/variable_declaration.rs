use crate::{runtime::Runtime, ast::nodes::VariableDeclaration};

pub fn parse_variable_declaration(runtime: &mut Runtime, declaration: &VariableDeclaration) {
    for variable in declaration.declarations.iter() {
        let name = variable.id.name.clone();
        let literal = variable.init.clone();

        runtime.current_scope().variables.insert(name, literal);
    } 
}