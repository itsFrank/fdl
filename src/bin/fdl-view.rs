use fdl::core::{Prop, PropValue, Thing};
use fdl::lexer::Lexer;
use fdl::parser::{ParseError, Parser};

use ruscii::app::{App, Config, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::{env, fs};

fn make_err_string(err: &ParseError) -> String {
    return format!(
        "line {}:{} - {}",
        err.token_info.line, err.token_info.col, err.message
    );
}

struct FdlViewState {
    pub things: HashMap<String, Thing>,
    pub thing_state: HashMap<*const Thing, bool>,
}

impl FdlViewState {
    pub fn new(things: HashMap<String, Thing>) -> Self {
        let thing_state = RefCell::new(HashMap::<*const Thing, bool>::new());
        for (_, thing) in &things {
            thing.foreach(|thing, _| {
                thing_state
                    .borrow_mut()
                    .insert(thing as *const Thing, false);
            });
        }

        return Self {
            things: things,
            thing_state: thing_state.into_inner(),
        };
    }

    fn foreach_thing(&self, f: impl Fn(&Thing, usize) -> ()) {
        let f = &f;
        for (_, thing) in &self.things {
            thing.foreach(f);
        }
    }
}

fn main() -> Result<(), String> {
    let file_path = match (env::args().collect::<Vec<String>>()).get(1) {
        Some(file_path) => file_path.clone(),
        None => return Err("missing file_path argument, usage: [fdl-view <file_path>]".to_string()),
    };

    let file_source = match fs::read_to_string(file_path) {
        Ok(file_source) => file_source,
        Err(err) => return Err(err.to_string()),
    };

    let parser = match Parser::from_tokens(Lexer::new(file_source.as_str())) {
        Ok(parser) => parser,
        Err(err) => return Err(make_err_string(&err)),
    };

    let mut fdl_view_state = FdlViewState::new(parser.things);

    let mut app = App::config(Config::new().fps(std::u32::MAX));
    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        let line = Cell::new(1 as i32);
        let pencil = RefCell::new(Pencil::new(window.canvas_mut()));
        fdl_view_state.foreach_thing(|thing, depth| {
            pencil.borrow_mut().draw_text(
                &format!("{}{}", " ".repeat(4 * depth), thing.name),
                Vec2::xy(1, line.get()),
            );
            line.set(line.get() + 1);
        });
    });

    return Ok(());
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {}
}
