extern crate syntect;
extern crate termion;

mod data;
mod view;
mod controller; 

use std::fs::File;
use std::time::Duration;
use std::io::{stdin, stdout};

use termion::screen::*;
use termion::raw::IntoRawMode;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet, Style};
use syntect::parsing::SyntaxSet;

use data::editor_state::{StateApi, EditorState, Mode};
use data::piece_table::PieceTable;
use controller::events::InputModeMultiplexer;


fn main() {    
    // let mut term = view::terminal::create_terminal();
    
    let file_name = "test_file.py";

    let mut file = File::open(file_name)
                        .expect("Unable to open file");

    let mut state = EditorState::new(
        String::from(file_name),
        Mode::Navigate,
        0,
        PieceTable::new(&mut file),
        vec![]
    );

    // Syntax highlighting stuff
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let theme = theme_set.themes.get("Solarized (dark)").unwrap();

    let syntax = syntax_set.find_syntax_by_extension("py").unwrap();
    let mut highlighter = HighlightLines::new(syntax, theme);
    
    controller::events::event_loop(&mut highlighter, &mut state);
}
