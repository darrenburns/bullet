extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::EditorState;

use std::cmp;

static INFO_BAR_HEIGHT: usize = 1;

pub fn update_screen(screen: &RustBox, state: &EditorState) {
  screen.clear();
  screen.print(0, screen.height() - INFO_BAR_HEIGHT, rustbox::RB_BOLD, Color::Black, Color::White, &info_bar_text(&screen, &state));
  render_editor_content(&screen, &state);
  screen.set_cursor(state.cursor_pos.x as isize, state.cursor_pos.y as isize);
  screen.present();
}

pub fn render_editor_content(screen: &RustBox, state: &EditorState) {
  let upper_render_limit = state.scroll.v_scroll + 
    cmp::min(screen.height() - INFO_BAR_HEIGHT, state.content.lines.len());
  for y in state.scroll.v_scroll..upper_render_limit {
    if y - state.scroll.v_scroll < screen.height() - INFO_BAR_HEIGHT {
      screen.print(0, y - state.scroll.v_scroll, rustbox::RB_NORMAL, Color::White, Color::Black, &state.content.lines[y]);
    }
  }
}

fn info_bar_text(screen: &RustBox, state: &EditorState) -> String {
  let left_text = "Ctrl + Q to quit.";
  let file_name = "SomeFile.md";
  let cursor_pos_string = state.cursor_pos.to_string();
  format!("{0}{1} vscroll:{3}, sheight: {4} num_lines: {5} {2: >6$}", 
    left_text, 
    cursor_pos_string, 
    file_name,
    state.scroll.v_scroll,
    screen.height(),
    state.content.lines.len(),
    screen.width() - left_text.len() - cursor_pos_string.len())
}

