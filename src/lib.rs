pub mod lexer {
    use std::char;

    pub struct Lexer {
        source: Vec<char>,
        index: usize,
    }

    impl Lexer {
        pub fn new(source: &str) -> Self {
            return Self {
                source: source.chars().collect(),
                index: 0,
            };
        }

        pub fn peek(&mut self) -> char {
            if self.index >= self.source.len() {
                return '\0';
            }
            return self.source[self.index];
        }

        pub fn next(&mut self) -> String {
            return self.consume();
        }

        fn consume(&mut self) -> String {
            let c = self.peek();

            match c {
                'A'..='Z' | 'a'..='z' => return self.consume_word(),
                '\0' => return "".to_string(),
                _ => {
                    self.index = self.index + 1;
                    return c.to_string();
                }
            }
        }

        fn consume_word(&mut self) -> String {
            let mut token = String::new();

            while self.peek().is_alphanumeric() {
                token.push(self.peek());
                self.index = self.index + 1;
            }

            return token;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lexer::*;

    #[test]
    fn last_token_returns_empty_string() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next(), "");
        assert_eq!(lexer.next(), "");
    }

    #[test]
    fn next_token_returns_char() {
        let mut lexer = Lexer::new("{");
        assert_eq!(lexer.next(), "{");
    }

    #[test]
    fn chars_are_tokenized_into_words() {
        let mut lexer = Lexer::new("hello world");
        assert_eq!(lexer.next(), "hello");
    }

    #[test]
    fn chars_are_tokenized_into_words_including_trailing_numbers() {
        let mut lexer = Lexer::new("hello123 world");
        assert_eq!(lexer.next(), "hello123");
    }
}
