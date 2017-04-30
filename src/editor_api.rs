// This module should provide a high level API for interacting with the editor
// rather than the lower level functionality currently available via EditorState

/// Moves the cursor to the next line, creating it if it doesn't already exist.
/// Returns the newly active line.

use editor_view::ViewState;
use editor_state::{EditorState, CursorPosition, CursorBounds};

pub struct BulletApi<'a> {
  pub view: &'a mut Option<ViewState>,
  pub model: &'a mut EditorState
}

impl<'a> BulletApi<'a> {

  pub fn insert_char(&mut self, ch: &char, row: &usize, col: &usize) {
    self.model.content.insert_char(&ch, &col, &row);
  }

  pub fn cursor_right(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_right, ViewState::cursor_mv_right)
  }

  pub fn cursor_left(&mut self) -> Result<CursorPosition, CursorBounds> {
    self.cursor_move(EditorState::cursor_mv_left, ViewState::cursor_mv_left)
  }

  fn cursor_move<F, G>(&mut self, state_fn: F, view_fn: G) -> Result<CursorPosition, CursorBounds> 
    where F: Fn(&mut EditorState) -> Result<CursorPosition, CursorBounds>,
          G: Fn(&mut ViewState) -> () {
    state_fn(self.model)
          .map(|new_pos| {
            self.view.as_mut()
                      .map(|view| view_fn(view));
            new_pos
          })
    

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