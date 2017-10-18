use std::fmt;

use data::piece_table::PieceTable;

pub struct EditorState {
    file_name: String,
    mode: Mode,
    cursor_position: CursorPosition,
    piece_table: PieceTable,
}

impl EditorState {
    pub fn new(
        file_name: String,
        mode: Mode, 
        cursor_position: CursorPosition,
        piece_table: PieceTable
    ) -> Self {

        EditorState { file_name, mode, cursor_position, piece_table }
    }

    pub fn set_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }
}


pub trait StateApi {
    fn get_mode(&self) -> &Mode;
    fn get_editor_lines(&self) -> Vec<&str>;
}

impl StateApi for EditorState {

    fn get_mode(&self) -> &Mode {
        &self.mode
    }

    fn get_editor_lines(&self) -> Vec<&str> {
        self.piece_table.as_lines()
    }
}

// Coordinates of the cursor WITHIN the text (not the screen), 
// indexed from 0.
pub struct CursorPosition {
    x: usize,
    y: usize,
}

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition { x: 0, y: 0 }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Mode {
    Navigate,  // Designed for quick navigation of documents.
    Command,  // Press ':' to enter this mode, enter string to perform command.
    Insert,  // Press 'i' while in Navigate mode to enter insert mode, for updating files. 
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}