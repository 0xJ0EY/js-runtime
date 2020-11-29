use super::{Token, parsers::{parse_block_comments, parse_curly_brace, parse_identifier, parse_line_comment, parse_number, parse_operator, parse_parenthesis, parse_separator, parse_string, parse_terminator, parse_whitespace}};

pub struct Tokenizer {
    pub index: usize,
    pub file_content: Vec<char>,
}

impl Tokenizer {
    pub fn new(file_content: &String) -> Tokenizer {
        Tokenizer {
            index: 0,
            file_content: file_content.chars().collect()
        }
    }

    pub fn has_tokens(&self) -> bool {
        self.token().is_some()
    }

    pub fn token(&self) -> Option<&char> {
        self.file_content.get(self.index)
    }

    pub fn peek(&self) -> Option<&char> {
        self.file_content.get(self.index + 1)
    }

    pub fn peek_back(&self) -> Option<&char> {
        self.file_content.get(self.index - 1)
    }

    pub fn walk_back(&mut self) {
        self.index -= 1;
    }

    pub fn consume(&mut self) -> Option<&char> {
        let value = self.file_content.get(self.index);

        self.index += 1;

        return value;
    }
}

pub fn tokenize(file_content: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut tokenizer = Tokenizer::new(file_content);

    while tokenizer.has_tokens() {
        if parse_whitespace(&mut tokenizer).is_some() {
            continue;
        }

        if parse_line_comment(&mut tokenizer).is_some() {
            continue;
        }

        if parse_block_comments(&mut tokenizer).is_some() {
            continue;
        }

        let identifier = parse_identifier(&mut tokenizer);
        if identifier.is_some() {
            tokens.push(identifier.unwrap());
            continue;
        }

        let number = parse_number(&mut tokenizer);
        if number.is_some() {
            tokens.push(number.unwrap());
            continue;
        }

        let string = parse_string(&mut tokenizer);
        if string.is_some() {
            tokens.push(string.unwrap());
            continue;
        }

        let operator = parse_operator(&mut tokenizer);
        if operator.is_some() {
            tokens.push(operator.unwrap());
            continue;
        }

        let curly_brace = parse_curly_brace(&mut tokenizer);
        if curly_brace.is_some() {
            tokens.push(curly_brace.unwrap());
            continue;
        }

        let seperator = parse_separator(&mut tokenizer);
        if seperator.is_some() {
            tokens.push(seperator.unwrap());
            continue;
        }

        let parenthesis = parse_parenthesis(&mut tokenizer);
        if parenthesis.is_some() {
            tokens.push(parenthesis.unwrap());
            continue;
        }

        let terminator = parse_terminator(&mut tokenizer);
        if terminator.is_some() {
            tokens.push(terminator.unwrap());
            continue;
        }

        panic!("Unknown token: {} at index: {}", tokenizer.token().unwrap().clone().to_string(), tokenizer.index);
    }

    tokens
}
