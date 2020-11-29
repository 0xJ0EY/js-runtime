#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub range: (usize, usize)
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
    pub raw: String,
    pub range: (usize, usize)
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Literal,
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
pub enum AstNode {
    VariableDeclaration {
        inner: VariableDeclaration,
    },
    BlockStatement {
        inner: BlockStatement
    },
    FunctionDeclaration {
        inner: FunctionDeclaration
    },
}