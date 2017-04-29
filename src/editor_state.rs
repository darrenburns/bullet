#[derive(Clone)]
pub struct CursorPosition {
  pub active_line: usize,
  pub active_col: usize
}

#[derive(Clone)]
pub struct EditorState {
  pub content: EditorContent,
  pub position: CursorPosition
}

impl EditorState {

  pub fn new() -> EditorState {
    EditorState {
      content: EditorContent::new(),
      position: CursorPosition {active_line: 1, active_col: 1}
    }
  }

  pub fn cursor_mv_right(&mut self) {
    let new_col = self.position.active_col + 1;
    let new_row = self.position.active_line;
    self.set_position(CursorPosition {active_col: new_col, active_line: new_row});
  }

  pub fn cursor_mv_left(&mut self) {
    let new_col = self.position.active_col - 1;
    let new_row = self.position.active_line;
    self.set_position(CursorPosition {active_col: new_col, active_line: new_row});
  }

  pub fn cursor_mv_up(&mut self) {
    let new_col = self.position.active_col;
    let new_row = self.position.active_line - 1;

    if new_row < 1 {
      panic!("Attempted to move cursor to non-positive row")
    }

    self.set_position(CursorPosition {active_col: new_col, active_line: new_row});
    self.correct_cursor_line_boundary();
  }

  fn correct_cursor_line_boundary(&mut self) {
    let mut line_num = self.position.active_line;
    if !self.cursor_within_line_bounds() {
      self.cursor_to_end_of_line(&line_num);
    }
  }

  pub fn origin_cursor_x(&mut self) {
    self.position.active_col = 1;
  }

  pub fn set_position(&mut self, new_pos: CursorPosition) {
    // Coordinate ranges should be validated before calling this
    self.position = new_pos;
  }

  pub fn cursor_within_line_bounds(&self) -> bool {
    let current_line_len = self.get_line_by_line_number(&self.position.active_line).len();
    self.position.active_col <= current_line_len
  }

  pub fn get_line_by_line_number(&self, line_num: &usize) -> &str {
    &self.content.lines[line_num - 1]
  }

  pub fn cursor_to_end_of_line(&mut self, line_number: &usize) {
    let new_position = CursorPosition {
      active_col: self.content.get_line_by_line_number(line_number).len(),
      active_line: *line_number
    };
    self.set_position(new_position);
  }

  pub fn get_current_line_number(&self) -> usize {
    self.position.active_line
  }

}

#[derive(Default, Debug, Clone)]
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

  describe! cursor_movement {

    before_each {
      let mut state = EditorState::new();
      let line_two_content = "line two";
      state.content.insert_line(&2, line_two_content);
      state.content.insert_line(&3, "line three");
      state.content.insert_line(&4, "line four");
    }

    it "should initialise the line number to 1" {
      assert_eq!(state.get_current_line_number(), 1);
    }

    it "should initialise with a single line" {
      let some_state = EditorState::new();
      assert_eq!(some_state.content.lines.len(), 1);
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
