use std::char;

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    String,
    Number,
    Word,
    Symbol,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub struct TokenInfo {
    pub line: usize,
    pub col: usize,
}

pub struct Lexer {
    source: Vec<char>,
    index: usize,
    line: usize,
    last_line_index: usize,
}

impl Token {
    pub fn new(kind: TokenKind, literal: &str) -> Self {
        return Self {
            kind: kind,
            literal: literal.to_string(),
        };
    }
}

impl TokenInfo {
    pub fn new(line: usize, col: usize) -> Self {
        return Self {
            line: line,
            col: col,
        };
    }
}

impl Iterator for Lexer {
    type Item = (Token, TokenInfo);

    fn next(&mut self) -> Option<(Token, TokenInfo)> {
        return self.consume();
    }
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        return Self {
            source: source.chars().collect(),
            index: 0,
            line: 0,
            last_line_index: 0,
        };
    }

    pub fn peek(&self) -> char {
        return self.peek_offset(0);
    }

    pub fn peek_offset(&self, offset: isize) -> char {
        let peek_index = (self.index as isize) + offset;
        if peek_index < 0 {
            return '\0';
        }

        let item = self.source.get(peek_index as usize);
        match item {
            Some(c) => return c.clone(),
            None => return '\0',
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.line += 1;
                self.last_line_index = self.index + 1;
            }
            self.index += 1;
        }
    }

    fn consume(&mut self) -> Option<(Token, TokenInfo)> {
        self.skip_whitespace();

        let token_info = TokenInfo {
            line: self.line,
            col: (self.index - self.last_line_index),
        };

        let c = self.peek();
        match c {
            '0'..='9' => return Some((self.consume_number(), token_info)),
            'A'..='Z' | 'a'..='z' => return Some((self.consume_word(), token_info)),
            '"' => return Some((self.consume_string(), token_info)),
            '\0' => return None,
            _ => return Some((self.consume_char(), token_info)),
        }
    }

    fn consume_char(&mut self) -> Token {
        let c = self.peek();
        if c != '\0' {
            self.index += 1;
        }

        return Token {
            kind: TokenKind::Symbol,
            literal: c.to_string(),
        };
    }

    fn consume_word(&mut self) -> Token {
        let mut literal = String::new();

        while self.peek().is_alphanumeric() || self.peek() == '_' {
            literal.push(self.peek().clone());
            self.index += 1;
        }

        return Token {
            kind: TokenKind::Word,
            literal: literal,
        };
    }

    fn consume_number(&mut self) -> Token {
        let mut literal = String::new();

        while self.peek().is_numeric() || (self.peek() == '.' && self.peek_offset(1).is_numeric()) {
            literal.push(self.peek().clone());
            self.index += 1;
        }

        return Token {
            kind: TokenKind::Number,
            literal: literal,
        };
    }

    fn consume_string(&mut self) -> Token {
        // start with opening quote
        let mut literal = self.peek().to_string();
        self.index += 1;

        while self.peek() != '"' || self.peek_offset(-1) == '\\' {
            literal.push(self.peek().clone());
            self.index += 1;
        }

        // add closing quote
        literal.push(self.peek().clone());
        self.index += 1;

        return Token {
            kind: TokenKind::String,
            literal: literal,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_token_returns_empty_string() {
        let mut lexer = Lexer::new("");
        assert!(lexer.next().is_none());
        assert!(lexer.next().is_none());
        assert!(lexer.next().is_none());
    }

    #[test]
    fn whitespace_is_ignored() {
        let mut lexer = Lexer::new(" \t\n ");
        assert!(lexer.next().is_none());
    }

    #[test]
    fn next_token_returns_char() {
        let mut lexer = Lexer::new("{");
        assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "{"));
    }

    #[test]
    fn alphabetics_are_tokenized_into_words() {
        let mut lexer = Lexer::new("hello world");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Word, "hello")
        );
    }

    #[test]
    fn alphabetics_are_tokenized_into_words_including_trailing_numbers() {
        let mut lexer = Lexer::new("hello123 world");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Word, "hello123")
        );
    }

    #[test]
    fn alphabetics_are_tokenized_into_words_including_underscores() {
        let mut lexer = Lexer::new("hello_world");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Word, "hello_world")
        );
    }

    #[test]
    fn numerics_are_tokenized_into_numbers() {
        let mut lexer = Lexer::new("123 world");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Number, "123")
        );
    }

    #[test]
    fn numerics_are_tokenized_into_numbers_ignore_trailing_alphabetics() {
        let mut lexer = Lexer::new("123world");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Number, "123")
        );
    }

    #[test]
    fn alphabetics_after_numerics_are_separate_word_tokens() {
        let mut lexer = Lexer::new("123world");
        lexer.next().unwrap().0;
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Word, "world")
        );
    }

    #[test]
    fn numerics_include_decimals() {
        let mut lexer = Lexer::new("123.11");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Number, "123.11")
        );
    }

    #[test]
    fn numerics_ignore_decimals_not_followed_by_numeric() {
        let mut lexer = Lexer::new("123..11");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Number, "123")
        );
    }

    #[test]
    fn numerics_with_trailing_decimals_are_separate_tokens() {
        let mut lexer = Lexer::new("123.");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::Number, "123")
        );
        assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "."));
    }

    #[test]
    fn quotes_consume_as_one_token_until_closing_quote() {
        let mut lexer = Lexer::new("\"Hello World, 12 12.2!\"");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::String, "\"Hello World, 12 12.2!\"")
        );
    }

    #[test]
    fn escaped_quotes_dont_end_string() {
        let mut lexer = Lexer::new("\"Hello \\\" World\"");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::String, "\"Hello \\\" World\"")
        );
    }

    #[test]
    fn strings_include_newlines() {
        let mut lexer = Lexer::new("\"Hello \n World\"");
        assert_eq!(
            lexer.next().unwrap().0,
            Token::new(TokenKind::String, "\"Hello \n World\"")
        );
    }

    #[test]
    fn tokens_are_parsed_with_column_index() {
        let mut lexer = Lexer::new("hello world");
        assert_eq!(lexer.next().unwrap().1, TokenInfo::new(0, 0));
        assert_eq!(lexer.next().unwrap().1, TokenInfo::new(0, 6));
    }

    #[test]
    fn tokens_are_parsed_with_line_index_and_column_relative_to_line() {
        let mut lexer = Lexer::new("hello\nworld!\nline 3");
        assert_eq!(
            lexer.next().unwrap(),
            (Token::new(TokenKind::Word, "hello"), TokenInfo::new(0, 0))
        );
        assert_eq!(
            lexer.next().unwrap(),
            (Token::new(TokenKind::Word, "world"), TokenInfo::new(1, 0))
        );
        assert_eq!(
            lexer.next().unwrap(),
            (Token::new(TokenKind::Symbol, "!"), TokenInfo::new(1, 5))
        );
        assert_eq!(
            lexer.next().unwrap(),
            (Token::new(TokenKind::Word, "line"), TokenInfo::new(2, 0))
        );
        assert_eq!(
            lexer.next().unwrap(),
            (Token::new(TokenKind::Number, "3"), TokenInfo::new(2, 5))
        );
    }
}
