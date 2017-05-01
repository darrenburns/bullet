use editor_view::ViewState;
use editor_state::{EditorState, CursorPosition, CursorBounds};

pub struct BulletApi<'a> {
  pub view: &'a mut Option<ViewState>,
  pub model: &'a mut EditorState
}

impl<'a> BulletApi<'a> {

  pub fn cursor_right(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_right, ViewState::cursor_mv_right)
  }

  pub fn cursor_left(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_left, ViewState::cursor_mv_left)
  }

  pub fn cursor_down(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_down, ViewState::cursor_mv_down)
  }

  pub fn cursor_up(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_up, ViewState::cursor_mv_up)
  }

  pub fn cursor_origin_x(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_origin_x, ViewState::cursor_origin_x)
  }

  pub fn cursor_to_end_of_line(&mut self) -> Result<CursorPosition, CursorBounds> {
    let current_line_len = self.get_current_line().len();
    let new_pos = self.model.cursor_to_end_of_line();
    self.view.as_mut().unwrap().cursor_set_x(current_line_len);
    new_pos
  }

  fn cursor_move<F, G>(&mut self, state_fn: F, view_fn: G) -> Result<CursorPosition, CursorBounds> 
    where F: Fn(&mut EditorState) -> Result<CursorPosition, CursorBounds>,
          G: Fn(&mut ViewState) -> () {
    state_fn(self.model)
      .map(|new_pos| {
        self.view.as_mut().map(|view| view_fn(view));
        new_pos
      })
      .or_else(|err| self.cursor_to_end_of_line())
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