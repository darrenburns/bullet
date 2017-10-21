use std::fmt;

use data::piece_table::PieceTable;

pub struct EditorState {
    file_name: String,
    mode: Mode,
    pub cursor_index: usize,
    piece_table: PieceTable,
}

impl EditorState {
    pub fn new(
        file_name: String,
        mode: Mode, 
        cursor_index: usize,
        piece_table: PieceTable
    ) -> Self {

        EditorState { file_name, mode, cursor_index, piece_table }
    }

    pub fn set_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }
}


pub trait StateApi {
    fn get_mode(&self) -> &Mode;
    fn get_active_file_name(&self) -> &str;
    fn get_editor_lines(&self) -> Vec<&str>;
    fn get_file_length_in_chars(&self) -> usize;
    fn get_cursor_position(&self) -> CursorPosition;
    fn set_cursor_index(&mut self, new_index: usize);
}

// Coordinates of the cursor WITHIN the text (not the screen),
// indexed from 0.
pub struct CursorPosition {
    pub x: usize,
    pub y: usize,
}

impl StateApi for EditorState {

    fn get_mode(&self) -> &Mode {
        &self.mode
    }

    fn get_active_file_name(&self) -> &str {
        &self.file_name
    }

    fn get_editor_lines(&self) -> Vec<&str> {
        self.piece_table.as_lines()
    }

    fn get_file_length_in_chars(&self) -> usize {
        self.piece_table.get_pieces().iter()
            .map(|p| p.length)
            .sum()
    }

    fn get_cursor_position(&self) -> CursorPosition {
        let mut cursor_pos = CursorPosition { x: 0, y: 0 };
        let lines = self.get_editor_lines();
        let mut chars_seen = 0;
        for (y, line) in lines.iter().enumerate() {
            // Keep looking at lines, taking note of how many characters we look at
            let line_len = line.len() + 1;
            chars_seen += line_len;
            // If we've seen beyond the cursor index
            if chars_seen > self.cursor_index {
                let start_of_line = chars_seen - line_len;
                cursor_pos.x = self.cursor_index - start_of_line;
                cursor_pos.y = y;
                return cursor_pos;
            }
        }
        cursor_pos
    }

    fn set_cursor_index(&mut self, new_index: usize) {
        self.cursor_index = new_index;
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
