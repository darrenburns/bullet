extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::EditorState;

pub fn update_screen(screen: &RustBox, state: &EditorState) {
  screen.clear();
  screen.print(0, screen.height() - 1, rustbox::RB_BOLD, Color::Black, Color::White, &info_bar_text(&screen, &state));
  render_editor_content(&screen, &state);
  screen.set_cursor(state.cursor_pos.x, state.cursor_pos.y);
  screen.present();
}

pub fn render_editor_content(screen: &RustBox, state: &EditorState) {
  for y in 0..state.content.lines.len() {
    screen.print(0, y, rustbox::RB_NORMAL, Color::White, Color::Black, &state.content.lines[y]);
  }
}

fn info_bar_text(screen: &RustBox, state: &EditorState) -> String {
  format!("{0: <2$}{1: >2$}", state.cursor_pos, "SomeFile.md", (screen.width()) - 10)  // TODO: Update to subtract len of filename
}

