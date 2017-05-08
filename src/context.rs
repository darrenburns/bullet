use editor_view::ViewState;
use editor::{Editor, CursorPosition, CursorBounds};

pub trait EditorContext {
  fn cursor_right(&mut self) -> Result<CursorPosition, CursorBounds>;
  fn cursor_left(&mut self) -> Result<CursorPosition, CursorBounds>;
  fn cursor_down(&mut self) -> Result<CursorPosition, CursorBounds>;
  fn cursor_up(&mut self) -> Result<CursorPosition, CursorBounds>;
  fn cursor_origin_x(&mut self) -> Result<CursorPosition, CursorBounds>;
  fn cursor_to_end_of_current_line(&mut self) -> Result<CursorPosition, CursorBounds>;
}

pub struct BulletContext<'a> {
  pub view: &'a mut Option<ViewState>,
  pub model: &'a mut Editor
}

impl<'a> EditorContext for BulletContext<'a> {
  fn cursor_right(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(Editor::cursor_mv_right, ViewState::cursor_mv_right)
  }

  fn cursor_left(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(Editor::cursor_mv_left, ViewState::cursor_mv_left)
  }

  fn cursor_down(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(Editor::cursor_mv_down, ViewState::cursor_mv_down)
        .or_else(|err| match err {
          CursorBounds::RowOutOfBounds(_) => 
            self.cursor_to_end_of_current_line(),
          CursorBounds::ColumnOutOfBounds(_) => {
            let line_below = self.get_current_line_number() + 1;
            self.cursor_to_end_of_line(&line_below)
          }
        })
  }

  fn cursor_up(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(Editor::cursor_mv_up, ViewState::cursor_mv_up)
        .or_else(|err| match err {
          CursorBounds::RowOutOfBounds(_) => self.cursor_origin_x(),
          CursorBounds::ColumnOutOfBounds(_) => {
            let current_line = self.get_current_line_number();
            let next_line = if current_line > 1 {
              current_line - 1
            } else {
              1
            };
            self.cursor_to_end_of_line(&next_line)
          }
        })
  }

  fn cursor_origin_x(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(Editor::cursor_origin_x, ViewState::cursor_origin_x)
  }

  fn cursor_to_end_of_current_line(&mut self) -> Result<CursorPosition, CursorBounds> {
    let current_line_len = self.model.get_current_line().content.len();
    let new_pos = self.model.cursor_to_end_of_line();
    new_pos
  }
}

impl<'a> BulletContext<'a> {

  fn cursor_origin(&mut self) -> Result<CursorPosition, CursorBounds> {
    let active_line = self.model.position.active_line;
    self.model.set_position(CursorPosition {
      active_col: 1,
      active_line
    })
  }

  fn cursor_to_end_of_line(&mut self, new_line: &usize) -> Result<CursorPosition, CursorBounds> {
    self.model.position.active_line = *new_line;
    let new_line_len = self.model.get_current_line().content.len();
    self.cursor_to_end_of_current_line()
  }



  fn cursor_move<F, G>(&mut self, state_fn: F, view_fn: G) -> Result<CursorPosition, CursorBounds> 
    where F: Fn(&mut Editor) -> Result<CursorPosition, CursorBounds>,
          G: Fn(&mut ViewState, &mut Editor) -> () {
    let new_pos = state_fn(self.model);
    view_fn(self.view.as_mut().unwrap(), self.model);
    new_pos
  }

  pub fn insert_char(&mut self, ch: &char, row: &usize, col: &usize) {
    self.model.content.insert_char(&ch, &col, &row);
  }

  pub fn save_file(&mut self) {
    self.model.save_file();
  }

  pub fn insert_line_below(&mut self) {
    let current_line_number = self.get_current_line_number();
    self.model.content.insert_line(&(current_line_number + 1), "");
  }

  pub fn delete_char_back(&mut self) -> Result<CursorPosition, CursorBounds> {
    let active_line_num = self.model.position.active_line;
    // if we're at the beginning of a line
    if self.model.position.active_col == 1 && active_line_num > 1 {
        // save current line in var, and delete from editor
      let deleted_line = &self.model.content.delete_line(active_line_num);
      
      // move cursor to end of previous line
      self.cursor_up();
      let mut new_pos = self.cursor_to_end_of_current_line();
      // append line to end of prev line
      self.append_to_line(new_pos.as_mut().unwrap().active_line, &deleted_line.content);
      new_pos
    } else {
      // otherwise do normal backspace
      self.model.content.delete_char_behind(&self.model.position);
      self.cursor_left()
    }
  }

  pub fn append_to_line(&mut self, line_num: usize, append_str: &str) {
    self.model.content.append_to_line(line_num, append_str);
  }

  pub fn get_current_line_number(&self) -> usize {
    self.model.position.active_line
  }

  pub fn get_number_of_lines(&self) -> usize {
    self.model.content.lines.len()
  }

  pub fn activate_search_menu(&mut self) {
    self.view.as_mut().unwrap().activate_search_menu();
  }

  pub fn repaint(&mut self) {
    self.view.as_mut().unwrap().repaint(self.model);
  }
}

#[cfg(test)]
mod tests {
  pub use super::BulletContext
;

  describe! api {
    it "should be true" {
      assert!(true);
    }
  }
}