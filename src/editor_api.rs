// This module should provide a high level API for interacting with the editor
// rather than the lower level functionality currently available via EditorState

/// Moves the cursor to the next line, creating it if it doesn't already exist.
/// Returns the newly active line.

use editor_view::ViewState;
use editor_state::EditorState;

pub struct BulletApi<'a> {
  pub view: &'a mut Option<ViewState>,
  pub model: &'a mut EditorState
}

impl<'a> BulletApi<'a> {
  pub fn cursor_right(&mut self) -> usize {
      self.model.cursor_mv_right();
      self.view.as_mut().and_then(|mut view| Some(view.cursor_mv_right()));
      1
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