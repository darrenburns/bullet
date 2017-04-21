extern crate rustbox;

use rustbox::RustBox;

pub struct EditorState {
  pub cursor_pos: Coordinate
}

#[derive(Default)]
pub struct Coordinate {
  pub x: isize, 
  pub y: isize
}

impl EditorState {
  pub fn set_cursor_pos(&mut self, new_pos: Coordinate) {
    self.cursor_pos = new_pos;
  }
}