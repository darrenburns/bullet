extern crate rustbox;

use std::fmt;

use rustbox::RustBox;

#[derive(Default, Debug)]
pub struct Coordinate {
  pub x: isize, 
  pub y: isize
}

impl fmt::Display for Coordinate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x {}, y {}", self.x, self.y)
  }
}

pub struct EditorState {
  pub cursor_pos: Coordinate,
  pub content: EditorContent
}

impl EditorState {
  pub fn set_cursor_pos(&mut self, new_pos: Coordinate) {
    self.cursor_pos = new_pos;
  }
}

#[derive(Default, Debug)]
pub struct EditorContent {
  pub lines: Vec<String>
}