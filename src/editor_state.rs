extern crate rustbox;

use rustbox::RustBox;

pub struct EditorState {
  pub screen: RustBox,
  pub cursor_pos: Coordinate
}

#[derive(Default)]
pub struct Coordinate {
  pub x: isize, 
  pub y: isize
}

impl EditorState {
  pub fn set_cursor_x(&mut self, new_x: &isize) {
    self.cursor_pos.x = *new_x;
    self.screen.set_cursor(*new_x, self.cursor_pos.y);
  }
}