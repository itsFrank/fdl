use std::{collections::HashMap, vec};

use crate::string_utils::strip_quotes;

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
            value: PropValue::String(strip_quotes(literal.into().as_str()).to_string()),
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

    fn foreach_helper(thing: &Thing, depth: usize, f: &impl Fn(&Thing, usize) -> ()) {
        f(thing, depth);
        for (_, thing) in &thing.things {
            Self::foreach_helper(thing, depth + 1, f);
        }
    }

    fn foreach_parent_helper(
        thing: &Thing,
        parent: Option<&Thing>,
        depth: usize,
        f: &impl Fn(&Thing, Option<&Thing>, usize) -> (),
    ) {
        f(thing, parent, depth);
        for (_, child) in &thing.things {
            Self::foreach_parent_helper(child, Some(thing), depth + 1, f);
        }
    }

    pub fn foreach(&self, f: impl Fn(&Thing, usize) -> ()) {
        Self::foreach_helper(self, 0, &f);
    }

    pub fn foreach_parent(&self, f: impl Fn(&Thing, Option<&Thing>, usize) -> ()) {
        Self::foreach_parent_helper(self, None, 0, &f);
    }
}

impl<'a> IntoIterator for &'a Thing {
    type Item = &'a Thing;
    type IntoIter = vec::IntoIter<&'a Thing>;

    fn into_iter(self) -> Self::IntoIter {
        let things = Vec::<&'a Thing>::new();
        return things.into_iter();
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

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
        let prop = Prop::string_from_literal("name", "\"hello world\"");
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

    #[test]
    fn foreach_thing_traverses_thing_trees() {
        let mut thing = Thing::new("Hello");
        thing.add_thing(Thing::new("World"));
        thing.add_thing(Thing::new("Bye"));
        let ref mut world_thing = thing.things.get_mut("World").unwrap();
        world_thing.add_thing(Thing::new("Inner"));

        let vec = RefCell::new(Vec::<String>::new());
        thing.foreach(|thing, depth| {
            vec.borrow_mut().push(format!("{}-{}", thing.name, depth));
        });

        assert!(vec.borrow().contains(&"Hello-0".to_string()));
        assert!(vec.borrow().contains(&"World-1".to_string()));
        assert!(vec.borrow().contains(&"Inner-2".to_string()));
        assert!(vec.borrow().contains(&"Bye-1".to_string()));
        assert_eq!(vec.borrow().len(), 4);
    }

    #[test]
    fn foreach_parent_thing_traverses_thing_trees() {
        let mut thing = Thing::new("Hello");
        thing.add_thing(Thing::new("World"));

        let vec = RefCell::new(Vec::<String>::new());
        thing.foreach_parent(|thing, parent, _| {
            let parent_name = match parent {
                Some(parent) => parent.name.clone(),
                None => "".to_string(),
            };
            vec.borrow_mut()
                .push(format!("{}-{}", parent_name, thing.name));
        });
        assert!(vec.borrow().contains(&"-Hello".to_string()));
        assert!(vec.borrow().contains(&"Hello-World".to_string()));
        assert_eq!(vec.borrow().len(), 2);
    }
}
