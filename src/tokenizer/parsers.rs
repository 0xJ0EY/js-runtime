extern crate regex;
use regex::Regex;

use super::{Token, TokenType, tokenizer::Tokenizer};

pub fn parse_whitespace(tokenizer: &mut Tokenizer) -> Option<()> {
    let token = tokenizer.token().unwrap();

    if !is_whitespace(token) {
        return None;
    }

    tokenizer.consume();
    Some(())
}

fn is_whitespace(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn parse_identifier(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_identifier(token) {
        return None;
    }

    let mut value = String::new();
    let start = tokenizer.index;

    let mut consumed = tokenizer.consume();

    while consumed.is_some() && is_identifier(&consumed.unwrap()) {
        value.push(consumed.unwrap().clone());

        consumed = tokenizer.consume();
    }

    tokenizer.walk_back();

    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::Identifier,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_identifier(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-zA-Z]").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn parse_number(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_number(token) {
        return None;
    }

    let mut value = String::new();
    let start = tokenizer.index;

    let mut consumed = tokenizer.consume();

    while consumed.is_some() && is_number(&consumed.unwrap()) {
        value.push(consumed.unwrap().clone());

        consumed = tokenizer.consume();
    }

    tokenizer.walk_back();

    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::Number,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_number(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn parse_separator(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_separator(token) {
        return None;
    }

    let start = tokenizer.index;
    let value = tokenizer.consume()
        .unwrap()
        .clone()
        .to_string();
    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::Separator,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_separator(token: &char) -> bool {
    *token == '.' || *token == ','
}

pub fn parse_parenthesis(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_parenthesis(token) {
        return None;
    }

    let start = tokenizer.index;
    let value = tokenizer.consume()
        .unwrap()
        .clone()
        .to_string();
    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::Parenthesis,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_parenthesis(token: &char) -> bool {
    *token == '(' || *token == ')'
}

pub fn parse_terminator(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_terminator(token) {
        return None;
    }

    let start = tokenizer.index;
    let value = tokenizer.consume()
        .unwrap()
        .clone()
        .to_string();
    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::Terminator,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_terminator(token: &char) -> bool {
    *token == ';'
}

pub fn parse_curly_brace(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_curly_brace(token) {
        return None;
    }

    let start = tokenizer.index;
    let value = tokenizer.consume()
        .unwrap()
        .clone()
        .to_string();
    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::CurlyBraces,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_curly_brace(token: &char) -> bool {
    *token == '{' || *token == '}'
}

pub fn parse_operator(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token().unwrap();

    if !is_operator(token) {
        return None;
    }

    let mut value = String::new();
    let start = tokenizer.index;

    let mut consumed = tokenizer.consume();

    while consumed.is_some() && is_operator(&consumed.unwrap()) {
        value.push(consumed.unwrap().clone());

        consumed = tokenizer.consume();
    }

    tokenizer.walk_back();

    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::Operator,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

fn is_operator(token: &char) -> bool {
    match *token {
        '=' |
        '>' |
        '<' | 
        '!' |
        '+' |
        '-' |
        '/' |
        '*' |
        '%' |
        '&' |
        '|' |
        '^' |
        '~' => true,
        _ => false
    }
}

pub fn parse_line_comment(tokenizer: &mut Tokenizer) -> Option<()> {
    if !is_start_line_comment(tokenizer) {
        return None;
    }

    let mut consumed = tokenizer.consume();

    while consumed.is_some() && !is_end_line_comment(consumed.unwrap()) {
        consumed = tokenizer.consume();
    }

    Some(())
}

fn is_start_line_comment(tokenizer: &Tokenizer) -> bool {
    let first_part = tokenizer.token().unwrap();
    let second_part = tokenizer.peek().unwrap_or(&' ');

    *first_part == '/' && *second_part == '/'
}

fn is_end_line_comment(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\n").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn parse_block_comments(tokenizer: &mut Tokenizer) -> Option<()> {
    if !is_start_block_comment(tokenizer) {
        return None;
    }

    let mut consumed = tokenizer.consume();

    while consumed.is_some() && !is_end_block_comment(tokenizer) {
        consumed = tokenizer.consume();
    }

    tokenizer.consume(); // Consume last *
    tokenizer.consume(); // Consume last /

    Some(())
}

fn is_start_block_comment(tokenizer: &Tokenizer) -> bool {
    let first_part = tokenizer.token().unwrap();
    let second_part = tokenizer.peek().unwrap_or(&' ');

    *first_part == '/' && *second_part == '*'
}

fn is_end_block_comment(tokenizer: &Tokenizer) -> bool {
    let first_part = tokenizer.token().unwrap();
    let second_part = tokenizer.peek().unwrap_or(&' ');

    *first_part == '*' && *second_part == '/'
}

pub fn parse_string(tokenizer: &mut Tokenizer) -> Option<Token> {
    let token = tokenizer.token();
    let delimiter = is_start_string(token.unwrap());

    if delimiter.is_none() {
        return None;
    }

    let mut raw_value = String::new();
    let mut value = String::new();
    let start = tokenizer.index;
    
    let delimiter = delimiter.unwrap();
    raw_value.push(delimiter.clone());

    let mut consumed = tokenizer.consume();

    while consumed.is_some() && !is_end_string(tokenizer, delimiter) {
        raw_value.push(tokenizer.token().unwrap().clone());
        value.push(tokenizer.token().unwrap().clone());

        // Do not let the escape char go into the normal value string
        if is_escape_char(tokenizer.token().unwrap()) &&
            is_delimiter(tokenizer.peek().unwrap_or(&' ')) {
            value.pop();
        }

        consumed = tokenizer.consume();
    }

    tokenizer.consume();
    raw_value.push(delimiter.clone());

    let end = tokenizer.index;

    Some(Token {
        token_type: TokenType::String,
        raw_value,
        value,
        range: (start, end)
    })
}

fn is_delimiter(token: &char) -> bool {
    *token == '\'' || *token == '\"'
}

fn is_escape_char(token: &char) -> bool {
    *token == '\\'
}

fn is_start_string(token: &char) -> Option<char> {
    if is_delimiter(token) {
        return Some(token.clone());
    }

    None
}

fn is_end_string(tokenizer: &Tokenizer, delimiter: char) -> bool {
    if is_escaped(tokenizer) { return false; }

    let token = tokenizer.token().unwrap();

    token.clone() == delimiter
}

fn is_escaped(tokenizer: &Tokenizer) -> bool {
    tokenizer.peek_back()
        .unwrap_or(&' ')
        .clone() == '\\'
}