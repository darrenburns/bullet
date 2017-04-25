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
    line_number: 1,
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
          Key::Up if state.line_number > 1 => {
            state.cursor_line_up();
          }
          Key::Down => {
            state.cursor_line_down();
            // If we're on the last line, create a new line on pressing down
            if state.line_number == state.content.lines.len() + 1 {
              state.content.insert_line(&state.line_number, "");
              state.origin_cursor_x();
            }
          }
          Key::Char(ch) => {
            state.content.insert_char(&ch, &state.cursor_pos.x, &state.line_number);
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

