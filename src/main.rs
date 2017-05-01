#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate rustbox;
#[macro_use] extern crate quick_error;

mod editor_state;
mod editor_view;
mod editor_api;

use std::error::Error;
use std::default::Default;

use editor_state::EditorState;
use editor_view::ViewState;
use editor_api::BulletApi;

use rustbox::Key;

fn main() {
  let view: ViewState = ViewState::new();
  let state: EditorState = EditorState::new();
  main_loop(state, view);
}

fn main_loop(mut state: EditorState, mut view: ViewState) {

  let mut bullet_api: BulletApi = BulletApi {
    view: &mut Some(view), 
    model: &mut state
  };


  loop {
    bullet_api.repaint();

    match bullet_api.view.as_mut().unwrap().screen.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {

        let line_number = bullet_api.model.position.active_line;

        match key {
          Key::Ctrl('q') => { break; }

          Key::Right => {
            bullet_api.cursor_right();
          }
          Key::Left => {
            bullet_api.cursor_left();
          }
          Key::Up => {
            bullet_api.cursor_up();
          }
          Key::Down => {
            bullet_api.cursor_down();
          }
          Key::Char(ch) => {
            let col = bullet_api.model.position.active_col;
            bullet_api.insert_char(&ch, &line_number, &col);
            bullet_api.cursor_right();
          }
          Key::Enter => {
            bullet_api.insert_line_below();
            bullet_api.cursor_down();
            bullet_api.cursor_origin_x();
          }
          _ => {}
        }
      },
      Err(e) => panic!("{}", e.description()),
      _ => { }
    }
  }
}

