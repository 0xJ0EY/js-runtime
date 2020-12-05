extern crate regex;
use regex::Regex;

pub fn is_whitespace(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn is_identifier(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-zA-Z]").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn is_number(token: &char) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]").unwrap();
    }
        
    RE.is_match(&(*token).to_string())
}

pub fn is_separator(token: &char) -> bool {
    *token == '.' || *token == ','
}

pub fn is_parenthesis(token: &char) -> bool {
    *token == '(' || *token == ')'
}

pub fn is_terminator(token: &char) -> bool {
    *token == ';'
}

pub fn is_curly_brace(token: &char) -> bool {
    *token == '{' || *token == '}'
}

pub fn is_operator(token: &char) -> bool {
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

pub fn is_escape_char(token: &char) -> bool {
    *token == '\\'
}

pub fn is_string_delimiter(token: &char) -> bool {
    *token == '\'' || *token == '\"'
}
