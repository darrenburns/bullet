extern crate rustbox;

mod editor_state;
mod editor_view;

use std::error::Error;
use std::default::Default;

use editor_state::EditorState;
use editor_state::Coordinate;

use rustbox::RustBox;
use rustbox::Key;

fn main() {

  let screen = match RustBox::init(Default::default()) {
    Result::Ok(v) => v,
    Result::Err(e) => panic!("{}", e)
  };

  let mut state: EditorState = EditorState {
    cursor_pos: Coordinate {x: 0, y: 0},
    line_number: 0,
    scroll: Default::default(),
    content: Default::default(),
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
            if state.cursor_pos.y == state.content.lines.len() {
              state.content.insert_line(&state.cursor_pos.y, "");
              state.origin_cursor_x();
            }
          }
          Key::Char(ch) => {
            state.content.insert_char(&ch, &state.cursor_pos.x, &state.cursor_pos.y);
            state.inc_cursor_x();
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

