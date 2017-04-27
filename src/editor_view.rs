extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::EditorState;

use std::cmp;

static INFO_BAR_HEIGHT: usize = 1;

pub fn update_screen(state: &EditorState) {
  state.screen.clear();
  state.screen.print(0, state.screen.height() - INFO_BAR_HEIGHT, rustbox::RB_BOLD, Color::Black, Color::White, &info_bar_text(&state.screen, &state));
  render_editor_content(&state.screen, &state);
  state.screen.set_cursor(state.cursor_pos.x as isize, state.cursor_pos.y as isize);
  state.screen.present();
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
  let line_num = state.get_current_line_number();
  format!("{0}{1} vscroll:{3}, sheight: {4} line_num: {5} num_lines: {6} {2: >7$}", 
    left_text, 
    cursor_pos_string, 
    file_name,
    state.scroll.v_scroll,
    screen.height(),
    line_num,
    state.content.lines.len(),
    screen.width() - left_text.len() - cursor_pos_string.len())
}

