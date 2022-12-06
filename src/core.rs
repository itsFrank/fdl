use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum PropValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Err,
}

#[derive(Debug)]
pub struct Prop {
    pub name: String,
    pub value: PropValue,
}

#[derive(Debug)]
pub struct Thing {
    pub name: String,
    pub props: HashMap<String, Prop>,
    pub things: HashMap<String, Thing>,
}

impl Prop {
    pub fn new_err(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            value: PropValue::Err,
        };
    }

    pub fn int_from_literal(name: impl Into<String>, literal: impl Into<String>) -> Self {
        match literal.into().parse::<i32>() {
            Ok(val) => Self {
                name: name.into(),
                value: PropValue::Int(val),
            },
            Err(_) => Self::new_err(name),
        }
    }

    pub fn float_from_literal(name: impl Into<String>, literal: impl Into<String>) -> Self {
        match literal.into().parse::<f32>() {
            Ok(val) => Self {
                name: name.into(),
                value: PropValue::Float(val),
            },
            Err(_) => Self::new_err(name),
        }
    }

    pub fn bool_from_literal(name: impl Into<String>, literal: impl Into<String>) -> Self {
        match literal.into().parse::<bool>() {
            Ok(val) => Self {
                name: name.into(),
                value: PropValue::Bool(val),
            },
            Err(_) => Self::new_err(name),
        }
    }

    pub fn string_from_literal(name: impl Into<String>, literal: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            value: PropValue::String(literal.into()),
        };
    }
}

impl Thing {
    pub fn new(name: impl Into<String>) -> Self {
        return Thing {
            name: name.into(),
            props: HashMap::new(),
            things: HashMap::new(),
        };
    }

    pub fn add_prop(&mut self, prop: Prop) {
        self.props.insert(prop.name.clone(), prop);
    }

    pub fn add_thing(&mut self, thing: Thing) {
        self.things.insert(thing.name.clone(), thing);
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;

    #[test]
    fn prop_can_parse_int() {
        let prop = Prop::int_from_literal("name", "12");
        assert!(matches!(prop.value, PropValue::Int(12)));
    }

    #[test]
    fn prop_can_parse_float() {
        let prop = Prop::float_from_literal("name", "12.1");
        assert_eq!(prop.value, PropValue::Float(12.1));
    }

    #[test]
    fn prop_can_parse_bool() {
        let prop = Prop::bool_from_literal("name", "true");
        assert_eq!(prop.value, PropValue::Bool(true));
    }

    #[test]
    fn prop_can_parse_string() {
        let prop = Prop::string_from_literal("name", "hello world");
        assert_eq!(prop.value, PropValue::String("hello world".to_string()));
    }

    #[test]
    fn prop_recover_from_parse_errors_and_produce_err_props() {
        let int_prop = Prop::int_from_literal("name", "hello world");
        let float_prop = Prop::float_from_literal("name", "hello world");
        let bool_prop = Prop::bool_from_literal("name", "hello world");
        assert_eq!(int_prop.value, PropValue::Err);
        assert_eq!(float_prop.value, PropValue::Err);
        assert_eq!(bool_prop.value, PropValue::Err);
    }

    #[test]
    fn things_have_names() {
        let thing = Thing::new("thing");
        assert_eq!(thing.name, "thing");
    }

    #[test]
    fn props_can_be_added_to_things() {
        let mut thing = Thing::new("thing");
        thing.add_prop(Prop::string_from_literal("name", "hello world"));
        assert_eq!(thing.props.len(), 1);
        assert_eq!(
            thing.props.get("name").unwrap().value,
            PropValue::String("hello world".to_string())
        );
    }

    #[test]
    fn things_can_be_added_as_children_to_things() {
        let mut thing = Thing::new("thing");
        thing.add_thing(Thing::new("thing2"));
        assert_eq!(thing.things.len(), 1);
        assert_eq!(thing.things.get("thing2").unwrap().name, "thing2");
    }
}
