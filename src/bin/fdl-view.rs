use std::collections::HashMap;
use std::{cmp, fmt, fs, io};

use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

use fdl::core::{ForeachCtrl, Thing};
use fdl::lexer::Lexer;
use fdl::parser::{ParseError, Parser};

fn make_err_string(err: &ParseError) -> String {
    return format!(
        "line {}:{} - {}",
        err.token_info.line, err.token_info.col, err.message
    );
}

type ThingKey = *const Thing;

struct FdlViewState {
    pub select_index: usize,
    pub thing_state: HashMap<ThingKey, bool>,
}

impl FdlViewState {
    pub fn new(things: &Vec<Thing>) -> Self {
        let mut thing_state = HashMap::<ThingKey, bool>::new();
        for thing in things {
            thing.foreach(|thing, _, depth| {
                thing_state.insert(thing as ThingKey, depth == 0);
            });
        }

        return Self {
            select_index: 1,
            thing_state: thing_state,
        };
    }

    pub fn is_open(&self, key: ThingKey) -> bool {
        return *self.thing_state.get(&key).unwrap_or(&false);
    }

    pub fn open_thing(&mut self, key: ThingKey) {
        // if !self.thing_state.contains_key(&key) {
        //     return;
        // }
        self.thing_state.insert(key, true);
    }

    pub fn close_thing(&mut self, key: ThingKey) {
        // if !self.thing_state.contains_key(&key) {
        //     return;
        // }
        self.thing_state.insert(key, false);
    }

    fn update_index(&mut self, value: i32, num_visible: usize) {
        let new_index = self.select_index as i32 + value;
        let new_index = cmp::max(new_index, 1) as usize;
        let new_index = cmp::min(new_index, num_visible);
        self.select_index = new_index;
    }
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

fn foreach_thing_ctrl(
    things: &Vec<Thing>,
    mut f: impl FnMut(&Thing, Option<&Thing>, usize) -> ForeachCtrl,
) {
    for thing in things {
        thing.foreach_ctrl(&mut f);
    }
}

fn print_props(thing: &Thing, pencil: &mut Pencil, x_offset: usize) {
    let mut line = 1;
    for (name, prop) in &thing.props {
        let text = format!("{}: {}", name, prop.value.to_string());
        pencil.draw_text(&text, Vec2::xy(x_offset, line + 2));
        line += 1;
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

    let mut max_height: usize = 0;
    let mut longest_thing_row: usize = 8;
    let mut longest_prop_row: usize = 7;
    foreach_thing(&things, |thing, _, depth| {
        max_height += 1;

        let len: usize = (depth * 4) + 2 + thing.name.len();
        longest_thing_row = cmp::max(longest_thing_row, len);

        for (name, prop) in &thing.props {
            let len = name.len() + 2 + prop.value.to_string().len();
            longest_prop_row = cmp::max(longest_prop_row, len);
        }
    });

    let max_height = max_height;
    let things_width = longest_thing_row + 2;
    let props_width = longest_prop_row + 2;
    let props_x_offset = things_width + 2;

    let mut app = App::new();
    app.run(|app_state: &mut State, window: &mut Window| {
        let mut pencil = Pencil::new(window.canvas_mut());

        pencil.draw_text("Things", Vec2::xy(1, 1));
        pencil.draw_text("Props", Vec2::xy(props_x_offset, 1));
        pencil.draw_hline('-', Vec2::xy(1, 2), props_width + things_width + 1);

        let mut line = 1;
        let mut selected_thing: ThingKey = std::ptr::null();
        foreach_thing_ctrl(&things, |thing, parent, depth| {
            if let Some(parent) = parent {
                if !fdl_view_state.is_open(parent) {
                    return ForeachCtrl::BreakSubtree;
                }
            }

            if fdl_view_state.select_index == line {
                selected_thing = thing;
                print_props(thing, &mut pencil, props_x_offset);
                pencil.set_background(Color::White);
                pencil.set_foreground(Color::Black);
            }

            let caret = if fdl_view_state.is_open(thing) {
                "⌄"
            } else {
                "›"
            };
            let caret = if thing.num_things() == 0 { "-" } else { caret };
            let text = format!("{} {}", caret, thing.name);
            pencil.draw_text(&text, Vec2::xy((depth * 4) + 1, line.clone() + 2));
            line += 1;

            pencil.set_background(Color::Black);
            pencil.set_foreground(Color::White);

            return ForeachCtrl::Continue;
        });

        pencil.draw_vline('|', Vec2::xy(things_width, 1), max_height + 2);

        let num_visible_things = line - 1;
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Up) => fdl_view_state.update_index(-1, num_visible_things),
                KeyEvent::Pressed(Key::Down) => fdl_view_state.update_index(1, num_visible_things),
                KeyEvent::Pressed(Key::Left) => fdl_view_state.close_thing(selected_thing),
                KeyEvent::Pressed(Key::Right) => fdl_view_state.open_thing(selected_thing),
                _ => (),
            }
        }
    });

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_index_can_increment() {
        let mut state = FdlViewState::new(&Vec::new());
        state.update_index(1, 3);
        assert_eq!(state.select_index, 2);
    }
}
