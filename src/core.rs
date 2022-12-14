use std::collections::HashMap;

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
    things: HashMap<String, Thing>,
}

pub struct ThingBuilder {
    thing: Thing,
}

pub enum ForeachCtrl {
    Break,
    BreakSubtree,
    Continue,
}

impl PropValue {
    pub fn to_string(&self) -> String {
        return match self {
            PropValue::Int(val) => val.to_string(),
            PropValue::Float(val) => val.to_string(),
            PropValue::Bool(val) => val.to_string(),
            PropValue::String(val) => val.clone(),
            PropValue::Err => "Err".to_string(),
        };
    }
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

    pub fn build(name: impl Into<String>) -> ThingBuilder {
        return ThingBuilder {
            thing: Thing::new(name),
        };
    }

    pub fn add_prop(&mut self, prop: Prop) {
        self.props.insert(prop.name.clone(), prop);
    }

    pub fn num_things(&self) -> usize {
        return self.things.len();
    }

    pub fn add_thing(&mut self, thing: Thing) -> Option<Thing> {
        return self.things.insert(thing.name.clone(), thing);
    }

    pub fn get_thing(&self, key: impl Into<String>) -> Option<&Thing> {
        return self.things.get(&key.into());
    }

    pub fn get_thing_mut(&mut self, key: impl Into<String>) -> Option<&mut Thing> {
        return self.things.get_mut(&key.into());
    }

    fn foreach_helper(
        thing: &Thing,
        parent: Option<&Thing>,
        depth: usize,
        f: &mut impl FnMut(&Thing, Option<&Thing>, usize) -> (),
    ) {
        f(thing, parent, depth);
        for (_, child) in &thing.things {
            Self::foreach_helper(child, Some(thing), depth + 1, f);
        }
    }

    pub fn foreach(&self, mut f: impl FnMut(&Thing, Option<&Thing>, usize) -> ()) {
        Self::foreach_helper(self, None, 0, &mut f);
    }

    fn foreach_ctrl_helper(
        thing: &Thing,
        parent: Option<&Thing>,
        depth: usize,
        f: &mut impl FnMut(&Thing, Option<&Thing>, usize) -> ForeachCtrl,
    ) -> ForeachCtrl {
        match f(thing, parent, depth) {
            ForeachCtrl::Break => return ForeachCtrl::Break,
            ForeachCtrl::BreakSubtree => return ForeachCtrl::BreakSubtree,
            _ => {}
        };

        for (_, child) in &thing.things {
            match Self::foreach_ctrl_helper(child, Some(thing), depth + 1, f) {
                ForeachCtrl::Break => return ForeachCtrl::Break,
                _ => {}
            }
        }
        return ForeachCtrl::Continue;
    }

    pub fn foreach_ctrl(&self, mut f: impl FnMut(&Thing, Option<&Thing>, usize) -> ForeachCtrl) {
        Self::foreach_ctrl_helper(self, None, 0, &mut f);
    }
}

impl ThingBuilder {
    pub fn thing(mut self, thing: Thing) -> ThingBuilder {
        self.thing.add_thing(thing);
        return self;
    }

    pub fn things(mut self, things: Vec<Thing>) -> ThingBuilder {
        for thing in things {
            self.thing.add_thing(thing);
        }
        return self;
    }

    pub fn prop(mut self, prop: Prop) -> ThingBuilder {
        self.thing.add_prop(prop);
        return self;
    }

    pub fn finish(self) -> Thing {
        return self.thing;
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
        assert_eq!(thing.num_things(), 1);
        assert_eq!(thing.get_thing("thing2").unwrap().name, "thing2");
    }

    #[test]
    fn foreach_thing_traverses_thing_trees() {
        let thing = Thing::build("Hello")
            .things(vec![
                Thing::build("World").thing(Thing::new("Inner")).finish(),
                Thing::new("Bye"),
            ])
            .finish();

        let mut vec = Vec::<String>::new();
        thing.foreach(|thing, parent, depth| {
            let parent_name = match parent {
                Some(thing) => thing.name.clone(),
                None => "".to_string(),
            };
            vec.push(format!("{}-{}-{}", thing.name, parent_name, depth));
        });

        assert!(vec.contains(&"Hello--0".to_string()));
        assert!(vec.contains(&"World-Hello-1".to_string()));
        assert!(vec.contains(&"Inner-World-2".to_string()));
        assert!(vec.contains(&"Bye-Hello-1".to_string()));
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn foreach_ctrl_breaksubtree_exits_traversal() {
        let thing = Thing::build("Hello")
            .things(vec![
                Thing::build("World").thing(Thing::new("Inner")).finish(),
                Thing::new("Bye"),
            ])
            .finish();

        let mut vec = Vec::<String>::new();
        thing.foreach_ctrl(|thing, _, _| {
            vec.push(thing.name.clone());

            if thing.name == "World" {
                return ForeachCtrl::BreakSubtree;
            }
            return ForeachCtrl::Continue;
        });

        assert!(vec.contains(&"Hello".to_string()));
        assert!(vec.contains(&"World".to_string()));
        assert!(vec.contains(&"Bye".to_string()));
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn foreach_ctrl_break_exits_traversal() {
        let thing = Thing::build("Hello")
            .things(vec![
                Thing::build("World").thing(Thing::new("Inner")).finish(),
                Thing::new("Bye"),
            ])
            .finish();

        let mut vec = Vec::<String>::new();
        thing.foreach_ctrl(|thing, _, _| {
            vec.push(thing.name.clone());

            if thing.name == "World" {
                return ForeachCtrl::Break;
            }
            return ForeachCtrl::Continue;
        });

        assert!(vec.contains(&"Hello".to_string()));
        assert!(vec.contains(&"World".to_string()));
        assert!(vec.len() >= 2); // >= because traverse order is not guaranteed
    }
}
