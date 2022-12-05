use std::collections::HashMap;

use crate::{
    core::Thing,
    lexer::{Token, TokenInfo, TokenKind},
    string_utils::strip_quotes,
};

pub struct Parser {
    pub things: HashMap<String, Thing>,
    thing_name_stack: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        return Self {
            things: HashMap::new(),
            thing_name_stack: Vec::new(),
        };
    }

    pub fn from_tokens(mut tokens: impl Iterator<Item = (Token, TokenInfo)>) -> Self {
        let mut parser = Self::new();

        while let Some((ref token, _)) = tokens.next() {
            parser.add_token(token, &mut tokens);
        }

        return parser;
    }

    fn add_thing(&mut self, name_literal: &String) {
        let name = strip_quotes(name_literal.as_str()).to_string();
        self.things.insert(name.clone(), Thing::new(name.clone()));
        self.thing_name_stack.push(name);
    }

    fn add_token<'a, I>(&mut self, token: &Token, iter: &mut I)
    where
        I: Iterator<Item = (Token, TokenInfo)>,
    {
        match token.kind {
            TokenKind::Word => match token.literal.as_str() {
                "thing" => match iter.next() {
                    Some((ref next_token, _)) => {
                        if next_token.kind == TokenKind::String {
                            self.add_thing(&next_token.literal);
                        }
                    }
                    None => {}
                },
                _ => {}
            },
            _ => return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn populate_parser(source: &str) -> Parser {
        let lexer = Lexer::new(source);
        return Parser::from_tokens(lexer);
    }

    #[test]
    fn parser_parses_an_empty_thing() {
        let parser = populate_parser(r#"thing "MyThing" {} "#);
        assert_eq!(parser.things.len(), 1);
        assert_eq!(parser.things.get("MyThing").unwrap().name, "MyThing");
    }
}
