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

  let screen = match::RustBox::init(Default::default()) {
    Result::Ok(v) => v,
    Result::Err(e) => panic!("{}", e)
  };
  editor_view::render_initial_screen(&screen);

  let mut state: EditorState = EditorState {
    cursor_pos: Coordinate {x: 0, y: 0} 
  }; 
  main_loop(&mut state, &screen);
}

fn main_loop(state: &mut EditorState, screen: &RustBox) {
  loop {
    match screen.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {
        match key {
          Key::Ctrl('q') => { break; }

          Key::Right => {
            let new_x = state.cursor_pos.x + 1;
            let new_y = state.cursor_pos.y;
            state.set_cursor_pos(Coordinate {x: new_x, y: new_y});
          }
          Key::Left => {
            let new_x = state.cursor_pos.x - 1;
            let new_y = state.cursor_pos.y;
            state.set_cursor_pos(Coordinate {x: new_x, y: new_y});
          }
          Key::Up => {
            let new_x = state.cursor_pos.x;
            let new_y = state.cursor_pos.y - 1;
            state.set_cursor_pos(Coordinate {x: new_x, y: new_y});
          }
          Key::Down => {
            let new_x = state.cursor_pos.x;
            let new_y = state.cursor_pos.y + 1;
            state.set_cursor_pos(Coordinate {x: new_x, y: new_y});
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

