extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::EditorState;

pub fn update_screen(screen: &RustBox, state: &EditorState) {
  screen.clear();
  screen.print(0, screen.height() - 1, rustbox::RB_BOLD, Color::Black, Color::White, &info_bar_text(&screen, &state));
  render_editor_content(&screen, &state);
  screen.set_cursor(state.cursor_pos.x as isize, state.cursor_pos.y as isize);
  screen.present();
}

pub fn render_editor_content(screen: &RustBox, state: &EditorState) {
  for y in 0..state.content.lines.len() {
    screen.print(0, y, rustbox::RB_NORMAL, Color::White, Color::Black, &state.content.lines[y]);
  }
}

fn info_bar_text(screen: &RustBox, state: &EditorState) -> String {
  let left_text = "Ctrl + Q to quit.";
  let file_name = "SomeFile.md";
  let cursor_pos_string = state.cursor_pos.to_string();
  format!("{0}{1}{2: >3$}", left_text, cursor_pos_string, file_name, 
    screen.width() - left_text.len() - cursor_pos_string.len())
}

