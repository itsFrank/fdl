use std::collections::HashMap;

use crate::{
    core::Thing,
    lexer::{Token, TokenInfo, TokenKind},
    string_utils::strip_quotes,
};

#[derive(Debug)]
pub struct ParseError {
    pub token_info: TokenInfo,
    pub message: String,
}

#[derive(Debug)]
pub struct Parser {
    pub things: HashMap<String, Thing>,
    thing_stack: Vec<Thing>,
}

impl ParseError {
    pub fn new(token_info: TokenInfo, message: impl Into<String>) -> Self {
        return Self {
            token_info: token_info,
            message: message.into(),
        };
    }
}

impl Parser {
    pub fn new() -> Self {
        return Self {
            things: HashMap::new(),
            thing_stack: Vec::new(),
        };
    }

    pub fn from_tokens(
        mut tokens: impl Iterator<Item = (Token, TokenInfo)>,
    ) -> Result<Self, ParseError> {
        let mut parser = Self::new();

        while let Some(item) = tokens.next() {
            match parser.parse_token(item, &mut tokens) {
                Ok(()) => {}
                Err(err) => return Err(err),
            }
        }

        if !parser.thing_stack.is_empty() {
            let thing = parser.thing_stack.pop().unwrap();
            return Err(ParseError::new(
                TokenInfo::new(0, 0),
                "Token `".to_owned() + &thing.name + "` is missing a closing brace `}`",
            ));
        }
        return Ok(parser);
    }

    fn add_thing(&mut self, name_literal: &String) {
        self.thing_stack
            .push(Thing::new(strip_quotes(&name_literal).to_string()));
    }

    fn parse_token<I>(
        &mut self,
        (token, token_info): (Token, TokenInfo),
        iter: &mut I,
    ) -> Result<(), ParseError>
    where
        I: Iterator<Item = (Token, TokenInfo)>,
    {
        match token.kind {
            TokenKind::Word => match token.literal.as_str() {
                "thing" => return self.parse_thing((token, token_info), iter),
                _ => {}
            },
            TokenKind::Symbol => match token.literal.as_str() {
                "}" => {
                    let Some(thing) = self.thing_stack.pop() else {
                        return Err(ParseError::new(token_info, "Unexpected symbol: `}`"));
                    };

                    match self.thing_stack.last_mut() {
                        Some(parent) => parent.things.insert(thing.name.clone(), thing),
                        None => self.things.insert(thing.name.clone(), thing),
                    };
                }
                _ => {}
            },
            _ => {}
        }
        return Ok(());
    }

    fn parse_thing<I>(
        &mut self,
        (_, token_info): (Token, TokenInfo),
        iter: &mut I,
    ) -> Result<(), ParseError>
    where
        I: Iterator<Item = (Token, TokenInfo)>,
    {
        let Some((token_p1, token_p1_info)) = iter.next() else {
            return Err(ParseError::new(token_info, "Expected String name after keyword `thing`"));
        };

        if token_p1.kind != TokenKind::String {
            return Err(ParseError::new(
                token_p1_info,
                "Expected String name after keyword `thing`",
            ));
        }

        let Some((token_p2, token_p2_info)) = iter.next() else {
            return Err(ParseError::new(token_p1_info, "Expected `{` after thing name"));
        };

        if token_p2.kind != TokenKind::Symbol || token_p2.literal != "{" {
            return Err(ParseError::new(
                token_p2_info,
                "Expected `{` after thing name",
            ));
        }

        self.add_thing(&token_p1.literal);
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn populate_parser(source: &str) -> Result<Parser, ParseError> {
        let lexer = Lexer::new(source);
        return Parser::from_tokens(lexer);
    }

    #[test]
    fn parses_a_thing() {
        let parser = populate_parser(r#"thing "MyThing" {} "#).unwrap();
        assert_eq!(parser.things.len(), 1);
        assert_eq!(parser.things.get("MyThing").unwrap().name, "MyThing");
    }

    #[test]
    fn parses_sibbling_things() {
        let parser = populate_parser(
            r#"
            thing "MyThing" {}
            thing "MyThing2" {}
        "#,
        )
        .unwrap();
        assert_eq!(parser.things.len(), 2);
    }

    #[test]
    fn parses_nested_things() {
        let parser = populate_parser(
            r#"
            thing "MyThing" {
                thing "Nested" {}
            } 
        "#,
        )
        .unwrap();
        assert_eq!(parser.things.len(), 1);
        assert_eq!(parser.things.get("MyThing").unwrap().things.len(), 1);
    }

    #[test]
    fn thing_as_last_token_leads_to_error() {
        assert!(populate_parser(r#"thing"#).is_err());
    }

    #[test]
    fn thing_not_followed_by_string_leads_to_error() {
        let err = populate_parser(r#"thing 12"#).unwrap_err();
        assert_eq!(err.token_info, TokenInfo::new(0, 6));
    }

    #[test]
    fn thing_and_name_without_opening_brace_leads_to_error() {
        let err = populate_parser(r#"thing "Name" a"#).unwrap_err();
        assert_eq!(err.token_info, TokenInfo::new(0, 13));
    }

    #[test]
    fn thing_without_closing_brace_leads_to_error() {
        let err = populate_parser(r#"thing "Name" {"#).unwrap_err();
        assert_eq!(err.token_info, TokenInfo::new(0, 0));
    }
}
