#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate rustbox;
#[macro_use] extern crate quick_error;

mod editor;
mod editor_view;
mod context;
mod line_buffer;
mod widget;

use std::env;
use std::error::Error;
use std::default::Default;

use editor::Editor;
use editor_view::ViewState;
use context::{BulletContext, EditorContext};

use rustbox::Key;

fn main() {
  let args: Vec<String> = env::args().collect();
  let view: ViewState = ViewState::new();
  let mut state: Editor = Editor::new();
  
  if args.len() == 2 {
    state.open_file(&args[1]);
  }

  main_loop(state, view);
}

fn main_loop(mut state: Editor, mut view: ViewState) {

  let mut context: BulletContext = BulletContext {
    view: &mut Some(view), 
    model: &mut state
  };
  context.repaint();
  
  loop {

    match context.view.as_mut().unwrap().screen.poll_event(false) {
      Ok(rustbox::Event::KeyEvent(key)) => {

        let line_number = context.model.position.active_line;

        match key {
          Key::Ctrl('q') => { break; }
          Key::Ctrl('s') => {
            context.save_file();
          }
          Key::Ctrl('f') => {
            context.activate_search_menu();
          }

          Key::Right => {
            context.cursor_right();
          }
          Key::Left => {
            context.cursor_left();
          }
          Key::Up => {
            context.cursor_up();
          }
          Key::Down => {
            context.cursor_down();
          }
          Key::Char(ch) => {
            let col = context.model.position.active_col;
            context.insert_char(&ch, &line_number, &col);
            context.cursor_right();
          }
          Key::Enter => {
            context.insert_line_below();
            context.cursor_down();
            context.cursor_origin_x();
          }
          Key::Backspace => {
            context.delete_char_back();
          }
          _ => {}
        }
      },
      Err(e) => panic!("{}", e.description()),
      _ => { }
    }
    
    context.repaint();
  }
}

