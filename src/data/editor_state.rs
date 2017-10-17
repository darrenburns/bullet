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
}


pub trait StateApi {
    fn get_editor_lines(&self) -> Vec<&str>;
}

impl StateApi for EditorState {
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

pub enum Mode {
    Normal,
    Insert,
}