extern crate rustbox;

mod editor_state;
mod editor_view;

use std::io::Read;

use std::error::Error;
use std::default::Default;

use editor_state::EditorState;
use editor_state::Coordinate;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {

  let screen = match RustBox::init(Default::default()) {
    Result::Ok(v) => v,
    Result::Err(e) => panic!("{}", e)
  };

  let mut state: EditorState = EditorState {
    cursor_pos: Coordinate {x: 0, y: 0},
    content: Default::default()
  };
  state.content.lines.push("".to_string());
  editor_view::update_screen(&screen, &state); 
  main_loop(&mut state, &screen);
}

fn main_loop(state: &mut EditorState, screen: &RustBox) {
  loop {
    match screen.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {
        match key {
          Key::Ctrl('q') => { break; }

          Key::Right if state.cursor_within_line_bounds() => {
            state.inc_cursor_x();
          }
          Key::Left if state.cursor_pos.x > 0 => {
            state.dec_cursor_x();
          }
          Key::Up if state.cursor_pos.y > 0 => {
            state.dec_cursor_y();
          }
          Key::Down if state.cursor_pos.y < state.content.lines.len() => {
            state.inc_cursor_y();
            state.content.insert_line(&state.cursor_pos.y, "");
          }
          Key::Char(ch) => {
            let new_x = state.cursor_pos.x + 1;
            let new_y = state.cursor_pos.y;
            state.content.insert_char(&ch, &state.cursor_pos.x, &state.cursor_pos.y);
            state.set_cursor_pos(Coordinate {
              x: new_x, 
              y: new_y
            });
          }
          _ => {}
        }
      },
      Err(e) => panic!("{}", e.description()),
      _ => { }
    }
    editor_view::update_screen(&screen, &state);
  }
}

