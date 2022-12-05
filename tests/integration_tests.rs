use fdl::lexer::*;

#[test]
fn tokenize_a_complex_source() {
    let source = r#"
        thing "Thing Name" {
            int int_prop = 12
            float float_prop = 12.1
            string str_prop = "I'm a String"
        }
    "#;

    let mut lexer = Lexer::new(source);
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Word, "thing")
    );
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::String, "\"Thing Name\"")
    );
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "{"));
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Word, "int"));
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Word, "int_prop")
    );
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "="));
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Number, "12"));
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Word, "float")
    );
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Word, "float_prop")
    );
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "="));
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Number, "12.1")
    );
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Word, "string")
    );
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::Word, "str_prop")
    );
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "="));
    assert_eq!(
        lexer.next().unwrap().0,
        Token::new(TokenKind::String, r#""I'm a String""#)
    );
    assert_eq!(lexer.next().unwrap().0, Token::new(TokenKind::Symbol, "}"));
    assert!(lexer.next().is_none());
}
