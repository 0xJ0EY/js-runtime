use crate::tokenizer::TokenType;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub range: (usize, usize)
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct BinaryLiteral { // TODO: Change this to a binary literal
    pub value: String,
    pub raw_value: String,
    pub range: (usize, usize)
}

#[derive(Debug)]
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
    Identifier(Identifier),
    MemberExpression(MemberExpression),
}

#[derive(Debug)]
pub struct MemberExpression {
    pub object: Box<MemberExpressionObject>,
    pub property: Identifier,
    pub range: (usize, usize),
}

#[derive(Debug)]
pub enum MemberExpressionObject {
    Identifier(Identifier),
    MemberExpression(MemberExpression),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: ExpressionStatementExpression,
    pub range: (usize, usize),
}

#[derive(Debug)]
pub enum ExpressionStatementExpression {
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
}

#[derive(Debug)]
pub enum AstNode {
    VariableDeclaration(VariableDeclaration),
    BlockStatement(BlockStatement),
    FunctionDeclaration(FunctionDeclaration),
    ExpressionStatement(ExpressionStatement),
}