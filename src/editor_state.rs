extern crate rustbox;

use std::fmt;
use std::cmp;

#[derive(Default, Debug, PartialEq)]
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

  pub fn new() -> EditorState {
    EditorState {
      cursor_pos: Coordinate {x: 0, y: 0},
      line_number: 1,
      scroll: Default::default(),
      content: EditorContent::new()
    }
  }

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

    if new_y >= self.content.lines.len() {
      panic!("Attempted to move cursor to non-existent line");
    }

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
    self.line_number = self.y_coord_to_line_num();
  }

  pub fn cursor_within_line_bounds(&self) -> bool {
    self.cursor_pos.x < self.content.lines[self.line_number - 1].len()
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

#[derive(Default, Debug, PartialEq)]
pub struct EditorScroll {
  pub v_scroll: usize,
  pub h_scroll: usize
}

#[derive(Default, Debug)]
pub struct EditorContent {
  pub lines: Vec<String>
}

impl EditorContent {

  pub fn new() -> EditorContent {
    EditorContent {
      lines: vec!["".to_string()]
    }
  }

  pub fn insert_char(&mut self, ch: &char, x: &usize, line_num: &usize) {
    let mut chars: Vec<char> = self.lines[*line_num-1].chars().collect();
    chars.insert(*x, *ch);
    self.lines[*line_num-1] = chars.into_iter().collect::<String>();
  }

  pub fn insert_line(&mut self, line_num: &usize, initial_content: &str) {
    self.lines.insert(*line_num - 1, initial_content.to_owned());
  }

}


#[cfg(test)]
mod tests {
  pub use super::EditorState;
  pub use super::EditorScroll;
  pub use super::Coordinate;

  describe! cursor_movement {

    before_each {
      let mut state = EditorState::new();
    }

    it "should initialise the line number to 1" {
      assert_eq!(state.line_number, 1);
    }

    it "should initialise with a single line" {
      assert_eq!(state.content.lines.len(), 1);
    }

    it "should initialise with no scrolling" {
      assert_eq!(state.scroll, EditorScroll{v_scroll:0, h_scroll:0});
    }

    it "should initialise with the cursor in the top left corner" {
      assert_eq!(state.cursor_pos, Coordinate{x:0, y:0});
    }

    it "should initialise with an empty line" {
      assert_eq!(state.content.lines[0], "");
    }

    it "should increment the cursor y value if the line below exists" {
      state.content.insert_line(&2, "");
      state.inc_cursor_y();
    }

    failing "should panic when attempting to move to a line that doesnt exist" {
      state.inc_cursor_y();
    }

    it "should increment the cursor x value" {
      state.inc_cursor_x();
      assert_eq!(state.cursor_pos.x, 1);
    }

    it "should decrement the cursor x value" {
      state.inc_cursor_x();
      state.dec_cursor_x();
      assert_eq!(state.cursor_pos.x, 0);
    }

    ignore "should move cursor to the end of a line when moving down from a longer line" {
      state.content.insert_line(&2, "aslkfdjlasjdf");
      state.content.insert_line(&3, "asd");
      state.inc_cursor_y();
      state.inc_cursor_y();
      assert_eq!(state.cursor_pos.x, 3);
    }

    ignore "should move cursor to the end of a line when moving up from a longer line" {
      state.content.insert_line(&2, "abcdef");
      state.content.insert_line(&3, "aasdfasdfasgdfsg");
      state.inc_cursor_y();
      state.inc_cursor_y();
      state.dec_cursor_y();
      assert_eq!(state.cursor_pos.x, 6);
    }
  }

  describe! cursor_position_calculations {
    before_each {
      let mut state = EditorState::new();
      state.content.insert_line(&2, "line two");
      state.content.insert_line(&3, "line three");
      state.content.insert_line(&4, "line four");
    }

    it "should know that the cursor lies within horizontal line boundary" {
      state.set_cursor_pos(Coordinate {
        x: 1,
        y: 2
      });
      assert_eq!(state.cursor_within_line_bounds(), true);
    }

    it "should determine if the cursor lies outwith horizontal line boundary" {
      state.set_cursor_pos(Coordinate {
        x: 999,
        y: 3
      });
      assert_eq!(state.cursor_within_line_bounds(), false);
    }

  }
}