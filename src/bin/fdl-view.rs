use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::{cmp, fmt, fs, io};

use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

use fdl::core::{Prop, PropValue, Thing};
use fdl::lexer::Lexer;
use fdl::parser::{ParseError, Parser};

fn make_err_string(err: &ParseError) -> String {
    return format!(
        "line {}:{} - {}",
        err.token_info.line, err.token_info.col, err.message
    );
}

struct FdlViewState {
    pub select_index: usize,
    pub num_visible: usize,
    pub thing_state: HashMap<*const Thing, bool>,
}

impl FdlViewState {
    pub fn new(things: &Vec<Thing>) -> Self {
        let thing_state = RefCell::new(HashMap::<*const Thing, bool>::new());
        let mut num_visible = things.len();
        for thing in things {
            num_visible += thing.things.len();
            thing.foreach(|thing, _, depth| {
                thing_state
                    .borrow_mut()
                    .insert(thing as *const Thing, depth == 0);
            });
        }

        return Self {
            select_index: 1,
            thing_state: thing_state.into_inner(),
            num_visible: num_visible,
        };
    }

    pub fn open_thing(&mut self, thing: &Thing) {
        let key = thing as *const Thing;
        if !self.thing_state.contains_key(&key) {
            return;
        }
        self.thing_state.insert(key, true);
        self.num_visible += thing.things.len();
    }

    pub fn close_thing(&mut self, thing: &Thing) {
        let key = thing as *const Thing;
        let open = self.thing_state.get(&key).unwrap_or(&false);
        if !open {
            return;
        }
        self.thing_state.insert(key, false);
        for (_, child) in &thing.things {
            self.close_thing(child);
        }
        self.num_visible -= thing.things.len();
    }

    fn update_index(&mut self, value: i32) {
        let new_index = self.select_index as i32 + value;
        let new_index = cmp::max(new_index, 1) as usize;
        let new_index = cmp::min(new_index, self.num_visible.clone());
        self.select_index = new_index;
    }
}

enum Command {
    OpenThing,
    CloseThing,
    CursorUp,
    CursorDown,
    Exit,
}

fn parse_file(file_path: String) -> Result<Vec<Thing>, String> {
    let file_source = match fs::read_to_string(file_path) {
        Ok(file_source) => file_source,
        Err(err) => return Err(err.to_string()),
    };

    let parser = match Parser::from_tokens(Lexer::new(file_source.as_str())) {
        Ok(parser) => parser,
        Err(err) => return Err(make_err_string(&err)),
    };

    return Ok(parser.things.into_values().collect());
}

fn foreach_thing(things: &Vec<Thing>, mut f: impl FnMut(&Thing, Option<&Thing>, usize) -> ()) {
    for thing in things {
        thing.foreach(&mut f);
    }
}

#[derive(Debug, Clone)]
struct FdlError {
    message: String,
}

impl fmt::Display for FdlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return f.write_str(&self.message);
    }
}

impl From<String> for FdlError {
    fn from(message: String) -> Self {
        return FdlError { message: message };
    }
}

impl From<&str> for FdlError {
    fn from(message: &str) -> Self {
        return FdlError {
            message: message.to_string(),
        };
    }
}

impl From<io::Error> for FdlError {
    fn from(err: io::Error) -> Self {
        return err.to_string().into();
    }
}

fn main() -> Result<(), FdlError> {
    // let file_path = match (env::args().collect::<Vec<String>>()).get(1) {
    //     Some(file_path) => file_path.clone(),
    //     None => return Err("missing file_path argument, usage: [fdl-view <file_path>]".to_string()),
    // };
    let file_path = ".\\resources\\sample.fdl".to_string();

    let things = parse_file(file_path)?;
    let mut fdl_view_state = FdlViewState::new(&things);

    let mut app = App::new();
    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        let mut pencil = Pencil::new(window.canvas_mut());
        let mut line = 1;
        foreach_thing(&things, |thing, _, depth| {
            let text = format!("> {}", thing.name);
            pencil.draw_text(&text, Vec2::xy((depth * 4) + 1, line.clone()));
            line += 1;
        });
    });

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_index_can_increment() {
        let mut state = FdlViewState::new(&Vec::new());
        state.num_visible = 3;
        state.update_index(1);
        assert_eq!(state.select_index, 2);
    }
}
