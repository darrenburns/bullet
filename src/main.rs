extern crate rustbox;

mod editor_state;

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

  screen.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "SomeFile.md");
  screen.present();

  let mut state: EditorState = EditorState {
    screen: screen, 
    cursor_pos: Coordinate {x: 0, y: 0} 
  }; 
  main_loop(&mut state);
}

fn main_loop(state: &mut EditorState) {
  loop {
    match state.screen.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {
        match key {
          Key::Ctrl('q') => { break; }

          Key::Char(ch) => {
            let new_x = state.cursor_pos.x + 1;
            state.set_cursor_x(&new_x);
          }
          _ => { }
        }
      },
      Err(e) => panic!("{}", e.description()),
      _ => { }
    }
    state.screen.present();
  }
}
