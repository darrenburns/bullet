use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Error;

#[derive(Clone, Debug)]
pub struct CursorPosition {
  pub active_line: usize,
  pub active_col: usize
}

#[derive(Clone, Debug)]
pub struct EditorState {
  pub filename: String,
  pub content: EditorContent,
  pub position: CursorPosition
}

impl EditorState {

  pub fn new() -> EditorState {
    EditorState {
      filename: "".to_string(),
      content: EditorContent::new(),
      position: CursorPosition {active_line: 1, active_col: 1}
    }
  }

  pub fn open_file(&mut self, filename: &str) {
    let lines = BufReader::new(File::open(filename).unwrap())
      .lines()
      .map(|l| LineBuffer { content: l.unwrap(), is_dirty: false })
      .collect();
    self.filename = filename.to_string();
    self.content = EditorContent { lines };
    self.position = CursorPosition {active_line: 1, active_col: 1}
  }

  pub fn save_file(&mut self) {
    let path = Path::new(&self.filename);
    let mut file = File::create(&self.filename).unwrap();
    let content_as_string = self.content.lines
      .iter()
      .map(|l| l.content.clone())
      .collect::<Vec<_>>()
      .join("\n");

    // After saving, mark all lines as clean
    self.content.lines.iter_mut().map(|l| l.is_dirty = false);
    
    file.write_all(content_as_string.as_bytes()).unwrap();
  }

  pub fn cursor_mv_right(&mut self) -> Result<CursorPosition, CursorBounds> {
    let new_col = self.position.active_col + 1;
    let new_row = self.position.active_line;
    self.set_position(CursorPosition {
      active_col: new_col,
      active_line: new_row
    })
  }

  pub fn cursor_mv_left(&mut self) -> Result<CursorPosition, CursorBounds> {
    let new_col = self.position.active_col - 1;
    let new_row = self.position.active_line;
    self.set_position(CursorPosition {
      active_col: new_col,
      active_line: new_row
    })
  }

  pub fn cursor_mv_up(&mut self) -> Result<CursorPosition, CursorBounds> {
    let new_col = self.position.active_col;
    let new_row = self.position.active_line - 1;
    self.set_position(CursorPosition {
      active_col: new_col,
      active_line: new_row
    })
  }

  pub fn cursor_mv_down(&mut self) -> Result<CursorPosition, CursorBounds> {
    let new_col = self.position.active_col;
    let new_row = self.position.active_line + 1;
    self.set_position(CursorPosition {
      active_col: new_col,
      active_line: new_row
    })
  }

  pub fn cursor_origin_x(&mut self) -> Result<CursorPosition, CursorBounds> {
    let active_line = self.position.active_line;
    self.set_position(CursorPosition {
      active_col: 1,
      active_line
    })
  }

  pub fn cursor_to_end_of_line(&mut self) -> Result<CursorPosition, CursorBounds> {
    let new_col = self.get_current_line().content.len() + 1;
    let active_line = self.position.active_line;
    self.set_position(CursorPosition {
      active_col: new_col,
      active_line
    })
  }

  pub fn set_position(&mut self, new_pos: CursorPosition) -> Result<CursorPosition, CursorBounds> {
    if new_pos.active_line < 1 || new_pos.active_line > self.content.lines.len() {
      return Err(CursorBounds::RowOutOfBounds(""));
    }
    let line_len = self.get_line_by_line_number(&new_pos.active_line).content.len();
    if new_pos.active_col < 1 || new_pos.active_col > line_len + 1 {
      return Err(CursorBounds::ColumnOutOfBounds(""));
    }
    self.position = new_pos.clone();
    Ok(new_pos)
  }

  pub fn cursor_within_line_bounds(&self) -> bool {
    let current_line_len = self.get_line_by_line_number(&self.position.active_line).content.len();
    self.position.active_col <= current_line_len
  }

  pub fn get_line_by_line_number(&self, line_num: &usize) -> &LineBuffer {
    &self.content.lines[line_num - 1]
  }

  pub fn get_current_line_number(&self) -> usize {
    self.position.active_line
  }

  pub fn get_current_line(&mut self) -> &LineBuffer {
    self.content.get_line_by_line_number(&mut self.get_current_line_number())
  }

}

#[derive(Default, Debug, Clone)]
pub struct LineBuffer {
  pub content: String,
  pub is_dirty: bool
}


#[derive(Default, Debug, Clone)]
pub struct EditorContent {
  pub lines: Vec<LineBuffer>
}

impl EditorContent {

  pub fn new() -> EditorContent {
    EditorContent {
      lines: vec![
        LineBuffer {
          content: "".to_string(),
          is_dirty: false
        }
      ]
    }
  }

  pub fn insert_char(&mut self, ch: &char, col: &usize, line_num: &usize) {
    let mut chars: Vec<char> = self.lines[line_num-1].content.chars().collect();
    chars.insert(col-1, *ch);
    self.lines[line_num-1] = LineBuffer {
      content: chars.into_iter().collect::<String>(),
      is_dirty: true
    }
  }

  pub fn insert_line(&mut self, line_num: &usize, initial_content: &str) {
    self.lines.insert(line_num - 1, LineBuffer {
      content: initial_content.to_owned(),
      is_dirty: true
    });
  }

  pub fn delete_char_behind(&mut self, pos: &CursorPosition) {
    let ref mut active_line = self.lines[pos.active_line - 1];
    if pos.active_col > 1 {
      active_line.content.remove(pos.active_col - 2);
    }
    active_line.is_dirty = true;
  }

  pub fn append_to_line(&mut self, line_num: usize, append_str: &str) {
    let mut line = self.lines[line_num - 1].clone();
    line.content += append_str;
    self.lines[line_num - 1] = LineBuffer {
      content: line.content.to_string(),
      is_dirty: true
    }
  }

  pub fn delete_line(&mut self, line_number: usize) -> LineBuffer {
    let deleted_line = self.lines[line_number - 1].clone();
    self.lines.remove(line_number - 1);
    deleted_line
  }

  pub fn get_line_by_line_number(&self, line_number: &usize) -> &LineBuffer {
    &self.lines[line_number - 1]
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


quick_error! {
  #[derive(Debug)]
  pub enum CursorBounds {
    RowOutOfBounds(descr: &'static str) {
      description(descr)
    }

    ColumnOutOfBounds(descr: &'static str) {
      description(descr)
    }
  }
}