extern crate rustbox;

use std::fmt;

#[derive(Default, Debug)]
pub struct Coordinate {
  pub x: usize, 
  pub y: usize
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
  pub fn inc_cursor_x(&mut self) {
    let new_x = self.cursor_pos.x + 1;
    let new_y = self.cursor_pos.y;
    self.set_cursor_pos(Coordinate {x: new_x, y: new_y});
  }

  pub fn dec_cursor_x(&mut self) {
    let new_x = self.cursor_pos.x - 1;
    let new_y = self.cursor_pos.y;
    self.set_cursor_pos(Coordinate {x: new_x, y: new_y});
  }

  pub fn dec_cursor_y(&mut self) {
    let new_x = self.cursor_pos.x;
    let new_y = self.cursor_pos.y - 1;
    self.set_cursor_pos(Coordinate {x: new_x, y: new_y});
  }

  pub fn inc_cursor_y(&mut self) {
    let new_x = self.cursor_pos.x;
    let new_y = self.cursor_pos.y + 1;
    self.set_cursor_pos(Coordinate {x: new_x, y: new_y});
  }

  pub fn set_cursor_pos(&mut self, new_pos: Coordinate) {
    self.cursor_pos = new_pos;
  }

  pub fn cursor_within_line_bounds(&self) -> bool {
    self.cursor_pos.x < self.content.lines[self.cursor_pos.y].len()
  }
}

#[derive(Default, Debug)]
pub struct EditorContent {
  pub lines: Vec<String>
}

impl EditorContent {

  pub fn insert_char(&mut self, ch: &char, x: &usize, y: &usize) {
    let mut chars: Vec<char> = self.lines[*y].chars().collect();
    chars.insert(*x, *ch);
    self.lines[*y] = chars.into_iter().collect::<String>();
  }

  pub fn insert_line(&mut self, y: &usize, initial_content: &str) {
    self.lines.insert(*y, initial_content.to_owned());
  }

}