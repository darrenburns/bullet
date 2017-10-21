use std::fmt;

use data::piece_table::PieceTable;

pub struct EditorState {
    file_name: String,
    mode: Mode,
    cursor_index: usize,
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
    fn get_cursor_position(&self) -> CursorPosition;
    fn set_cursor_index(&mut self, new_index: usize);
}

// Coordinates of the cursor WITHIN the text (not the screen),
// indexed from 0.
pub struct CursorPosition {
    x: usize,
    y: usize,
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

    fn get_cursor_position(&self) -> CursorPosition {
        let lines = self.get_editor_lines();
        let mut chars_seen_until_line = 0;
        let mut cursor_pos = CursorPosition { x: 0, y: 0 };
        for (y, line) in lines.iter().enumerate() {
            let chars_this_line = line.len() + 1;
            let total_chars_seen = chars_this_line + chars_seen_until_line;
            if total_chars_seen >= self.cursor_index {
                let this_line_offset = total_chars_seen - self.cursor_index;
                cursor_pos.x = chars_seen_until_line + this_line_offset;
                cursor_pos.y = y;
            }
            chars_seen_until_line += chars_this_line;  // add 1 for new line char
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