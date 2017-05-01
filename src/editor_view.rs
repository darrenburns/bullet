extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::EditorState;

use std::cmp;
use std::fmt;

static INFO_BAR_HEIGHT: usize = 1;

#[derive(Default, Debug, PartialEq)]
pub struct Coordinate {
  pub x: usize, 
  pub y: usize
}

impl Coordinate {
  pub fn inc_x(&mut self) {
    self.x += 1;
  }
  
  pub fn inc_y(&mut self) {
    self.y += 1;
  }

  pub fn dec_x(&mut self) {
    self.x -= 1;
  }

  pub fn dec_y(&mut self) {
    self.y -= 1;
  }

  pub fn set_x(&mut self) {
    self.x = 0;
  }

  pub fn set_y(&mut self) {
    self.y = 0;
  }
}

impl fmt::Display for Coordinate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x {}, y {}", self.x, self.y)
  }
}

#[derive(Default, Debug, PartialEq)]
pub struct EditorScroll {
  pub v_scroll: usize,
  pub h_scroll: usize
}

impl EditorScroll {
  pub fn scroll_down(&mut self) {
    self.v_scroll += 1;
  }

  pub fn scroll_up(&mut self) {
    self.v_scroll -= 1;
  }
}

pub struct ViewState {
  pub cursor_coords: Coordinate,
  pub scroll: EditorScroll,
  pub screen: RustBox
}

impl ViewState {
  pub fn new() -> ViewState {
    let screen = match RustBox::init(Default::default()) {
      Result::Ok(v) => v,
      Result::Err(e) => panic!("{}", e)
    };
    ViewState {
      cursor_coords: Coordinate {x: 0, y: 0},
      screen: screen,
      scroll: Default::default()
    }
  }

  pub fn cursor_mv_right(&mut self) {
    if self.cursor_coords.x < self.screen.width() {
      self.cursor_coords.inc_x();
    }
  }

  pub fn cursor_mv_left(&mut self) {
    if self.cursor_coords.x > 0 {
      self.cursor_coords.dec_x();
    }
  }

  pub fn cursor_mv_up(&mut self) {
    if self.cursor_coords.y > 0 {
      self.cursor_coords.dec_y();
    } else {
      self.scroll.scroll_up();
    }
  }

  pub fn cursor_mv_down(&mut self) {
    if self.cursor_coords.y < self.screen.height() - INFO_BAR_HEIGHT - 1 {
      self.cursor_coords.inc_y();
    } else {
      self.scroll.scroll_down();
    }
  }

  pub fn cursor_origin_x(&mut self) {
    self.cursor_coords.x = 0;
  }

  pub fn cursor_set_x(&mut self, x: usize) {
    self.cursor_coords.x = x;
  }


  pub fn repaint(&mut self, latest_state: &EditorState) {
    self.screen.clear();
    self.render_info_bar(&latest_state);
    self.render_lines(&latest_state.content.lines);
    self.screen.set_cursor(self.cursor_coords.x as isize, self.cursor_coords.y as isize);
    self.screen.present();
  }

  fn render_lines(&mut self, lines: &Vec<String>) {
    let upper_render_limit = self.scroll.v_scroll + 
      cmp::min(self.screen.height() - INFO_BAR_HEIGHT, lines.len());
    for y in self.scroll.v_scroll..upper_render_limit {
      if y - self.scroll.v_scroll < self.screen.height() - INFO_BAR_HEIGHT {
        self.screen.print(0, y - self.scroll.v_scroll, rustbox::RB_NORMAL, Color::White, Color::Black, &lines[y]);
      }
    }
  }

  fn render_info_bar(&mut self, editor_state: &EditorState) {
    let info_text = format!("{0:?}", editor_state);
    self.screen.print(0, self.screen.height() - INFO_BAR_HEIGHT, rustbox::RB_BOLD, Color::Black, Color::White, &info_text);
  }

}
