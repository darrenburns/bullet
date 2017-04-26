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
    // Coordinate ranges should be validated before calling this
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
      x: self.content.get_line_by_line_number(line_number).len(),
      y: *line_number - 1
    };
    self.set_cursor_pos(new_coords);
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

  pub fn get_line_by_line_number(&mut self, line_number: &usize) -> &str {
    &self.lines[*line_number - 1]
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

    it "should be able to set the cursor to x=0 for the current line" {
      state.origin_cursor_x();
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

  describe! scrolling_and_boundaries {
    before_each {
      let mut state = EditorState::new();
      state.content.insert_line(&2, "line two");
      state.content.insert_line(&3, "line three");
      state.content.insert_line(&4, "line four");
    }

    it "should determine the x=0 is within the line boundary" {
      state.set_cursor_pos(Coordinate {
        x: 0,
        y: 2
      });
      assert!(state.cursor_within_line_bounds());
    }

    // TODO: Should really move to bottom of screen, not bottom line
    it "should move cursor to bottom line on attempt to move to non-existent line" {
      state.set_cursor_pos(Coordinate {
        x: 0,
        y: 99999
      });
      assert_eq!(state.line_number, 4);
    }

    it "should determine that the last character on the line is within the line boundary" {
      state.set_cursor_pos(Coordinate {
        x: 9,
        y: 2
      });
      assert!(state.cursor_within_line_bounds());
    }

    it "should know that the cursor lies within horizontal line boundary" {
      state.set_cursor_pos(Coordinate {
        x: 1,
        y: 2
      });
      assert!(state.cursor_within_line_bounds());
    }

    it "should determine if the cursor lies outwith horizontal line boundary" {
      state.set_cursor_pos(Coordinate {
        x: 999,
        y: 3
      });
      assert_eq!(state.cursor_within_line_bounds(), false);
    }

    // TODO: Need to encapsulate logic for getting/setting line number
    ignore "should calculate the line number correctly" {
      state.scroll.v_scroll = 20;
      state.cursor_pos.y = 1;
      assert_eq!(state.line_number, 22);
    }

  }

  describe! editing_content {
   
    before_each {
      let mut state = EditorState::new();
      let mut line_two_content = "line two";
      state.content.insert_line(&2, line_two_content);
      state.content.insert_line(&3, "line three");
      state.content.insert_line(&4, "line four");
    }

    it "should insert a line at the bottom of a document" {
      let line_content = "test";
      state.content.insert_line(&5, line_content);
      assert_eq!(state.content.get_line_by_line_number(&5), line_content);
    }

    it "should insert a line on in the document, and shift all below lines down one" {
      let line_content = "test";
      state.content.insert_line(&2, line_content);
      assert_eq!(state.content.get_line_by_line_number(&2), line_content);
      assert_eq!(state.content.get_line_by_line_number(&3), line_two_content);
    }

    it "should insert a new character at the start of a line" {
      state.content.insert_char(&'X', &0, &2);
      assert_eq!(state.content.get_line_by_line_number(&2), "X".to_string() + line_two_content);
    }

    it "should insert a new character at the end of a line" {
      state.content.insert_char(&'X', &(line_two_content.len()), &2);
      assert_eq!(state.content.get_line_by_line_number(&2),  line_two_content.to_string() + "X");
    }

    it "should insert a new character in the middle of a line, shifting characters on the right up an index" {
      state.content.insert_char(&'X', &(line_two_content.len() / 2), &2);
      assert_eq!(state.content.get_line_by_line_number(&2),  "lineX two");
    }

  }

}
