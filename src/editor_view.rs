extern crate rustbox;

use rustbox::{Color, RustBox};
use editor_state::{EditorState, LineBuffer};

use std::cmp;
use std::fmt;

static INFO_BAR_HEIGHT: usize = 1;
static GUTTER_PAD: usize = 3;

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

  pub fn set_x(&mut self, x: usize) {
    self.x = x;
  }

  pub fn set_y(&mut self, y: usize) {
    self.y = y;
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
    if self.v_scroll > 0 {
      self.v_scroll -= 1;
    }
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

  pub fn cursor_mv_right(&mut self, editor_state: &mut EditorState) {
    if self.cursor_coords.x < self.screen.width() {
      self.cursor_coords.inc_x();
    }
  }

  pub fn cursor_mv_left(&mut self, editor_state: &mut EditorState) {
    if self.cursor_coords.x > 0 {
      self.cursor_coords.dec_x();
    }
  }

  pub fn cursor_mv_up(&mut self, editor_state: &mut EditorState) {
    if self.cursor_coords.y == 0 {
      self.scroll.scroll_up();
    }
  }

  pub fn cursor_mv_down(&mut self, editor_state: &mut EditorState) {
    if self.cursor_coords.y == self.screen.height() - INFO_BAR_HEIGHT - 1 {
      self.scroll.scroll_down();
    }
  }

  pub fn set_cursor_x(&mut self, cursor_x: usize) {
    if cursor_x <= self.screen.width() {
      self.cursor_coords.set_x(cursor_x);
    }
  }

  pub fn cursor_origin_x(&mut self, editor_state: &mut EditorState) {
    self.cursor_coords.x = 0;
  }

  pub fn repaint(&mut self, latest_state: &EditorState) {
    let gutter_width = latest_state.content.lines.len().to_string().len();
    self.cursor_coords.x = latest_state.position.active_col - self.scroll.h_scroll - 1;
    self.cursor_coords.y = latest_state.position.active_line - self.scroll.v_scroll - 1;
    self.screen.clear();
    self.render_info_bar(&latest_state);
    self.render_lines(&latest_state.content.lines, gutter_width, latest_state.position.active_line);
    self.screen.set_cursor((gutter_width + GUTTER_PAD + self.cursor_coords.x) as isize, self.cursor_coords.y as isize);
    self.screen.present();
  }

  fn render_lines(&mut self, lines: &Vec<LineBuffer>, gutter_width: usize, active_line: usize) {
    let upper_render_limit = self.scroll.v_scroll + 
      cmp::min(self.screen.height() - INFO_BAR_HEIGHT, lines.len());
    for y in self.scroll.v_scroll..upper_render_limit {
      if y - self.scroll.v_scroll < self.screen.height() - INFO_BAR_HEIGHT && y < lines.len() {
        let gutter =  format!(" {line_num: >gut_width$}  ", 
          line_num = (y+1).to_string(),
          gut_width = gutter_width
        );
        let gutter_row_style = if y == active_line - 1 {
          rustbox::RB_BOLD
        } else {
          rustbox::RB_NORMAL
        };
        let gutter_render_colour = if y == active_line - 1 {
          rustbox::Color::Cyan
        } else {
          rustbox::Color::Magenta
        };
        self.screen.print(0, y - self.scroll.v_scroll, gutter_row_style, gutter_render_colour, Color::Black, &gutter);
        self.screen.print(gutter_width + GUTTER_PAD, y - self.scroll.v_scroll, rustbox::RB_NORMAL, Color::White, Color::Default, &lines[y].content);
      }
    }
  }

  fn render_info_bar(&mut self, editor_state: &EditorState) {
    let info_text_left = format!(" Editing {file_name}",
      file_name = editor_state.filename
    );
    
    let info_text_right = format!("Line {line_num:?} out of {line_count} lines. Column {col:?}. Scroll {scroll:?} ", 
      line_num = editor_state.position.active_line,
      line_count = editor_state.content.lines.len(),
      col = editor_state.position.active_col, 
      scroll = self.scroll.v_scroll
    );

    let info_bar = format!("{info_text_left: <half_screen_width$}{info_text_right: >half_screen_width$}", 
      info_text_left = info_text_left,
      info_text_right = info_text_right,
      half_screen_width = self.screen.width() / 2
    );
    self.screen.print(0, self.screen.height() - INFO_BAR_HEIGHT, rustbox::RB_BOLD, Color::White, Color::Magenta, &info_bar);
  }

}
