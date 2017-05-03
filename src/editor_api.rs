use editor_view::ViewState;
use editor_state::{EditorState, CursorPosition, CursorBounds};

pub struct BulletApi<'a> {
  pub view: &'a mut Option<ViewState>,
  pub model: &'a mut EditorState
}

impl<'a> BulletApi<'a> {

  pub fn cursor_right(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_right)
  }

  pub fn cursor_left(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_left)
  }

  pub fn cursor_down(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_down)
        // .or_else(|err| match err {
        //   CursorBounds::RowOutOfBounds(_) => self.cursor_origin_x(),
        //   CursorBounds::ColumnOutOfBounds(_) => {
        //     let line_above = self.get_current_line_number() - 1;
        //     self.cursor_to_end_of_line(&line_above)
        //   }
        // })
  }

  pub fn cursor_up(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.model.cursor_mv_up()
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

  pub fn cursor_origin_x(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_origin_x)
  }

  fn cursor_origin(&mut self) -> Result<CursorPosition, CursorBounds> {
    let active_line = self.model.position.active_line;
    self.model.set_position(CursorPosition {
      active_col: 1,
      active_line
    })
  }

  fn cursor_to_end_of_line(&mut self, new_line: &usize) -> Result<CursorPosition, CursorBounds> {
    let new_line_len = self.get_current_line().len();
    self.model.position.active_line = *new_line;
    self.cursor_to_end_of_current_line()
  }

  pub fn cursor_to_end_of_current_line(&mut self) -> Result<CursorPosition, CursorBounds> {
    let current_line_len = self.get_current_line().len();
    let new_pos = self.model.cursor_to_end_of_line();
    new_pos
  }

  fn cursor_move<F>(&mut self, state_fn: F) -> Result<CursorPosition, CursorBounds> 
    where F: Fn(&mut EditorState) -> Result<CursorPosition, CursorBounds> {
    state_fn(self.model)
  }

  pub fn insert_char(&mut self, ch: &char, row: &usize, col: &usize) {
    self.model.content.insert_char(&ch, &col, &row);
  }

  pub fn insert_line_below(&mut self) {
    let current_line_number = self.get_current_line_number();
    self.model.content.insert_line(&(current_line_number + 1), "");
  }

  pub fn get_current_line(&self) -> &str {
    self.model.get_current_line()
  }

  pub fn get_current_line_number(&self) -> usize {
    self.model.position.active_line
  }

  pub fn get_number_of_lines(&self) -> usize {
    self.model.content.lines.len()
  }

  pub fn repaint(&mut self) {
    self.view.as_mut().unwrap().repaint(self.model);
  }
}

#[cfg(test)]
mod tests {
  pub use super::BulletApi;

  describe! api {
    it "should be true" {
      assert!(true);
    }
  }
}