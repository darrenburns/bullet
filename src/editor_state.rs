extern crate rustbox;

use std::fmt;
use std::cmp;

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
  pub scroll: EditorScroll,
  pub content: EditorContent,
  pub line_number: usize
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
    self.line_number = self.y_coord_to_line_num();
    self.correct_cursor_line_boundary();
  }

  pub fn inc_cursor_y(&mut self) {
    let new_x = self.cursor_pos.x;
    let new_y = self.cursor_pos.y + 1;
    self.set_cursor_pos(Coordinate {x: new_x, y: new_y});
    self.line_number = self.y_coord_to_line_num();
    self.correct_cursor_line_boundary();
  }

  fn correct_cursor_line_boundary(&mut self) {
    if !self.cursor_within_line_bounds() {
      let line_num = self.line_number;
      self.cursor_to_end_of_line(&line_num);
    }
  }

  pub fn origin_cursor_x(&mut self) {
    self.cursor_pos.x = 0;
  }

  pub fn set_cursor_pos(&mut self, new_pos: Coordinate) {
    self.cursor_pos = new_pos;
  }

  pub fn cursor_within_line_bounds(&self) -> bool {
    self.cursor_pos.x < self.content.lines[self.line_number - 1].len()
        && self.cursor_pos.x > 0
  }

  pub fn y_coord_to_line_num(&self) -> usize {
    cmp::min(self.scroll.v_scroll + self.cursor_pos.y + 1, self.content.lines.len())
  }

  pub fn cursor_to_end_of_line(&mut self, line_number: &usize) {
    let new_coords = Coordinate {
      x: self.get_line_by_line_number(line_number).len(),
      y: *line_number - 1
    };
    self.set_cursor_pos(new_coords);
  }

  pub fn get_line_by_line_number(&mut self, line_number: &usize) -> &str {
    &self.content.lines[*line_number - 1]
  }

}

#[derive(Default)]
pub struct EditorScroll {
  pub v_scroll: usize,
  pub h_scroll: usize
}

#[derive(Default, Debug)]
pub struct EditorContent {
  pub lines: Vec<String>
}

impl EditorContent {

  pub fn insert_char(&mut self, ch: &char, x: &usize, line_num: &usize) {
    let mut chars: Vec<char> = self.lines[*line_num-1].chars().collect();
    chars.insert(*x, *ch);
    self.lines[*line_num-1] = chars.into_iter().collect::<String>();
  }

  pub fn insert_line(&mut self, line_num: &usize, initial_content: &str) {
    self.lines.insert(*line_num - 1, initial_content.to_owned());
  }

}