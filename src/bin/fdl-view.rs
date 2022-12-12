use fdl::core::{Prop, PropValue, Thing};
use fdl::lexer::Lexer;
use fdl::parser::{ParseError, Parser};

use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr::null;
use std::{cmp, env, fs};

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
            thing.foreach(|thing, depth| {
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

fn foreach_thing_parent(things: &Vec<Thing>, f: impl Fn(&Thing, Option<&Thing>, usize) -> ()) {
    for thing in things {
        thing.foreach_parent(&f);
    }
}

fn main() -> Result<(), String> {
    // let file_path = match (env::args().collect::<Vec<String>>()).get(1) {
    //     Some(file_path) => file_path.clone(),
    //     None => return Err("missing file_path argument, usage: [fdl-view <file_path>]".to_string()),
    // };
    let file_path = ".\\resources\\sample.fdl".to_string();

    let things = parse_file(file_path)?;
    let mut fdl_view_state = FdlViewState::new(&things);

    let mut app = App::config(Config::new().fps(std::u32::MAX));
    app.run(|app_state: &mut State, window: &mut Window| {
        let mut commands: Vec<Command> = Vec::new();
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Up) => commands.push(Command::CursorUp),
                KeyEvent::Pressed(Key::Down) => commands.push(Command::CursorDown),
                KeyEvent::Pressed(Key::Right) => commands.push(Command::OpenThing),
                KeyEvent::Pressed(Key::Left) => commands.push(Command::CloseThing),
                KeyEvent::Pressed(Key::Esc) => commands.push(Command::Exit),
                _ => (),
            }
        }

        for command in commands {
            match command {
                Command::CursorUp => fdl_view_state.update_index(-1),
                Command::CursorDown => fdl_view_state.update_index(1),
                Command::OpenThing => fdl_view_state.update_index(0),
                Command::CloseThing => fdl_view_state.update_index(0),
                Command::Exit => app_state.stop(),
            }
        }

        let line = Cell::new(1 as usize);
        let pencil = RefCell::new(Pencil::new(window.canvas_mut()));
        let selected: Cell<*const Thing> = Cell::new(null());
        foreach_thing_parent(&things, |thing, parent, depth| {
            if let Some(parent) = parent {
                let parent_open = fdl_view_state
                    .thing_state
                    .get(&(parent as *const Thing))
                    .unwrap_or(&false);
                if !parent_open {
                    return;
                }
            }

            let open = fdl_view_state
                .thing_state
                .get(&(thing as *const Thing))
                .unwrap_or(&false);

            if fdl_view_state.select_index == line.get() {
                pencil.borrow_mut().set_background(Color::White);
                pencil.borrow_mut().set_foreground(Color::Black);
                selected.set(thing as *const Thing);
            }
            pencil.borrow_mut().draw_text(
                &format!("{} {}", if *open { "⌄" } else { "›" }, thing.name,),
                Vec2::xy((4 * depth) + 1, line.get()),
            );
            pencil.borrow_mut().set_background(Color::Black);
            pencil.borrow_mut().set_foreground(Color::White);
            line.set(line.get() + 1);
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
