#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate rustbox;

mod editor_state;
mod editor_view;

use std::error::Error;
use std::default::Default;

use editor_state::EditorState;
use editor_state::Coordinate;

use rustbox::Key;

fn main() {
  let mut state: EditorState = EditorState::new();
  main_loop(&mut state);
}

fn main_loop(state: &mut EditorState) {
  loop {
    editor_view::update_screen(&state);
    match state.screen.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {

        let mut line_number = state.get_current_line_number();

        match key {
          Key::Ctrl('q') => { break; }

          Key::Right if state.cursor_within_line_bounds() => {
            state.inc_cursor_x();
          }
          Key::Left if state.cursor_pos.x > 0 => {
            state.dec_cursor_x();
          }
          Key::Up if line_number > 1 => {
            state.dec_cursor_y();
          }
          Key::Down => {
            if line_number == state.content.lines.len() {
              state.content.insert_line(&(line_number + 1), "");
              state.origin_cursor_x();
            }
          }
          Key::Char(ch) => {
            state.content.insert_char(&ch, &state.cursor_pos.x, &line_number);
            state.inc_cursor_x();
          }
          _ => {}
        }
      },
      Err(e) => panic!("{}", e.description()),
      _ => { }
    }
  }
}

