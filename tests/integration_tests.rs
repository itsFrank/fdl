use fdl::core::PropValue;
use fdl::lexer::*;
use fdl::parser::*;

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

#[test]
fn parse_a_complex_source() {
    let source = r#"
        thing "Thing Name" {
            int int_prop = 12
            float float_prop = 12.1
            bool bool_prop = true
            string string_prop = "I'm a String"
        }
    "#;

    let lexer = Lexer::new(source);
    let parser = Parser::from_tokens(lexer).unwrap();

    assert_eq!(parser.things.len(), 1);
    let thing = parser.things.get("Thing Name").unwrap();
    assert_eq!(thing.name, "Thing Name");
    assert_eq!(thing.things.len(), 0);
    assert_eq!(thing.props.len(), 4);

    let int_prop = thing.props.get("int_prop").unwrap();
    assert_eq!(int_prop.name, "int_prop");
    assert_eq!(int_prop.value, PropValue::Int(12));

    let float_prop = thing.props.get("float_prop").unwrap();
    assert_eq!(float_prop.name, "float_prop");
    assert_eq!(float_prop.value, PropValue::Float(12.1));

    let bool_prop = thing.props.get("bool_prop").unwrap();
    assert_eq!(bool_prop.name, "bool_prop");
    assert_eq!(bool_prop.value, PropValue::Bool(true));

    let string_prop = thing.props.get("string_prop").unwrap();
    assert_eq!(string_prop.name, "string_prop");
    assert_eq!(
        string_prop.value,
        PropValue::String("I'm a String".to_string())
    );
}

#[test]
fn parse_nested_things_with_props() {
    let source = r#"
        thing "Root" {
            int int_prop = 12
            
            thing "Child1" {
                bool bool_prop = true
            }

            thing "Child2" {
                float float_prop = 12.1
            }
        }
    "#;

    let lexer = Lexer::new(source);
    let parser = Parser::from_tokens(lexer).unwrap();

    assert_eq!(parser.things.len(), 1);
    let thing = parser.things.get("Root").unwrap();
    assert_eq!(thing.name, "Root");
    assert_eq!(thing.things.len(), 2);
    assert_eq!(thing.props.len(), 1);

    let int_prop = thing.props.get("int_prop").unwrap();
    assert_eq!(int_prop.name, "int_prop");
    assert_eq!(int_prop.value, PropValue::Int(12));

    let child1 = thing.things.get("Child1").unwrap();
    assert_eq!(child1.name, "Child1");
    assert_eq!(child1.things.len(), 0);
    assert_eq!(child1.props.len(), 1);

    let bool_prop = child1.props.get("bool_prop").unwrap();
    assert_eq!(bool_prop.name, "bool_prop");
    assert_eq!(bool_prop.value, PropValue::Bool(true));

    let child2 = thing.things.get("Child2").unwrap();
    assert_eq!(child2.name, "Child2");
    assert_eq!(child2.things.len(), 0);
    assert_eq!(child2.props.len(), 1);

    let float_prop = child2.props.get("float_prop").unwrap();
    assert_eq!(float_prop.name, "float_prop");
    assert_eq!(float_prop.value, PropValue::Float(12.1));
}
