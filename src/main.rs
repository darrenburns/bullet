extern crate rustty;

mod data;
mod view;
mod controller; 

use std::fs::File;
use std::time::Duration;

use data::editor_state::{StateApi, EditorState, CursorPosition, Mode};
use data::piece_table::PieceTable;


fn main() {    
    let mut term = view::terminal::create_terminal();
    
    let file_name = "test.txt";

    let mut file = File::open(file_name)
                        .expect("Unable to open file");

    let mut state = EditorState::new(
        String::from(file_name),
        Mode::Navigate,
        CursorPosition::default(),
        PieceTable::new(&mut file),
    );
    
    controller::events::event_loop(&mut term, &mut state);
}
