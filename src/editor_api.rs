// This module should provide a high level API for interacting with the editor
// rather than the lower level functionality currently available via EditorState

/// Moves the cursor to the next line, creating it if it doesn't already exist.
/// Returns the newly active line.
pub fn cursor_down(state: &mut EditorState) -> usize {
  state.inc_cursor_y();
}

#[cfg(test)]
mod tests {

}