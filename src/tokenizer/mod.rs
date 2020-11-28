mod tokenizer;
mod parsers;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub raw_value: String,
    pub value: String,
    pub range: (usize, usize),
}

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Operator,
    Number,
    String,
    Parenthesis,
    CurlyBraces,
    Separator,
    Terminator,
}

pub fn tokenize(file: &String) -> Vec<Token> {
    let tokens = tokenizer::tokenize(file);

    return tokens;
}
