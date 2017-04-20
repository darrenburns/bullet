extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::EditorState;

pub fn update_screen(screen: &RustBox, state: &EditorState) {
  screen.set_cursor(state.cursor_pos.x, state.cursor_pos.y);
  screen.present();
}

pub fn render_initial_screen(screen: &RustBox) {
  screen.print(0, screen.height() - 1, rustbox::RB_BOLD, Color::Black, Color::White, &format!("{0: >1$}", "SomeFile.md ", screen.width()));
  screen.set_cursor(0, 0);
  screen.present();
}
