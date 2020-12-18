use crate::tokenizer::TokenType;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub range: (usize, usize)
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: String,
    pub raw_value: String,
    pub range: (usize, usize)
}

impl Literal {
    pub fn new() -> Self {
        Self {
            value: "".to_string(),
            raw_value: "".to_string(),
            range: (0,0)
        }
    }

    pub fn from_str(str: &str) -> Self {
        Self {
            value: str.to_string(),
            raw_value: str.to_string(),
            range: (0,0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryLiteral { // TODO: Change this to a binary literal
    pub value: String,
    pub raw_value: String,
    pub range: (usize, usize)
}

#[derive(Debug, Clone)]
pub enum VariableLiteral {
    Literal(Literal),
    BinaryLiteral(BinaryLiteral)
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: VariableLiteral,
    pub range: (usize, usize)
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub range: (usize, usize)
}

#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<AstNode>,
    pub range: (usize, usize)
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
    pub range: (usize, usize)
}

#[derive(Debug)]
pub struct CallExpression {
    pub callee: CallExpressionCallee,
    pub arguments: Vec<Literal>,
    pub range: (usize, usize),
}

#[derive(Debug)]
pub enum CallExpressionCallee {
    Identifier(Box<Identifier>),
    MemberExpression(Box<MemberExpression>),
}

#[derive(Debug, Clone)]
pub struct MemberExpression {
    pub object: Option<Box<MemberExpression>>,
    pub property: Identifier,
    pub range: (usize, usize),
}

impl MemberExpression {
    pub fn new(property: Identifier) -> Self {
        let range = property.range.clone();
        
        Self {
            object: None,
            property,
            range
        }
    }
}

#[derive(Debug)]
pub enum MemberExpressionObject {
    Identifier(Box<Identifier>),
    MemberExpression(Box<MemberExpression>),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: CallExpression,
    pub range: (usize, usize),
}

#[derive(Debug)]
pub enum AstNode {
    VariableDeclaration(VariableDeclaration),
    BlockStatement(BlockStatement),
    FunctionDeclaration(FunctionDeclaration),
    ExpressionStatement(ExpressionStatement),
}