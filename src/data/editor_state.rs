use std::fmt;
use std::cmp;

use termion::event::Event;

use data::piece_table::PieceTable;
use controller::input::ExprState;
use controller::events::InputModeMultiplexer;

pub struct EditorState {
    file_name: String,
    mode: Mode,
    pub cursor_index: usize,
    piece_table: PieceTable,
    pub mode_input_buffer: Vec<char>,
    pub expression_state: ExprState,
}

impl EditorState {
    pub fn new(
        file_name: String,
        mode: Mode,
        cursor_index: usize,
        piece_table: PieceTable,
        mode_input_buffer: Vec<char>
    ) -> Self {
        let expression_state = ExprState::Waiting;
        EditorState { file_name, mode, cursor_index, piece_table, mode_input_buffer, expression_state }
    }

    pub fn set_mode(&mut self, new_mode: Mode) {
        self.mode = new_mode;
    }
}


pub trait StateApi {
    fn get_mode(&self) -> &Mode;
    fn get_mode_input_buffer(&self) -> &Vec<char>;
    fn get_active_file_name(&self) -> &str;
    fn get_editor_lines(&self) -> Vec<String>;
    fn get_file_length_in_chars(&self) -> usize;
    fn get_cursor_position(&self) -> CursorPosition;
    fn set_cursor_index(&mut self, new_index: usize);
    fn cursor_to_eof(&mut self);
    fn inc_cursor(&mut self);
    fn dec_cursor(&mut self);
    fn cursor_line_down(&mut self);
    fn cursor_line_up(&mut self);
    fn cursor_start_next_word(&mut self);
    fn cursor_start_prev_word(&mut self);  // See note about common logic @ implementations
    fn cursor_start_of_line(&mut self);
    fn cursor_end_of_line(&mut self);
    // Add function for going to first match before, first match after index given a Predicate
    // Add function for going to matching parens
}

// Coordinates of the cursor WITHIN the text (not the screen),
// indexed from 0.
#[derive(Debug)]
pub struct CursorPosition {
    pub x: usize,
    pub y: usize,
}

impl StateApi for EditorState {

    fn get_mode(&self) -> &Mode {
        &self.mode
    }

    fn get_mode_input_buffer(&self) -> &Vec<char> {
        &self.mode_input_buffer
    }

    fn get_active_file_name(&self) -> &str {
        &self.file_name
    }

    fn get_editor_lines(&self) -> Vec<String> {
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
            let line_len = line.len();
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

    fn cursor_to_eof(&mut self) {
        self.cursor_index = self.get_file_length_in_chars() - 1
    }

    fn inc_cursor(&mut self) {
        if self.cursor_index < self.get_file_length_in_chars() - 1 {
            self.cursor_index += 1;
        }
    }

    fn dec_cursor(&mut self) {
        if self.cursor_index >= 1 {
            self.cursor_index -= 1;
        }
    }

    fn cursor_line_down(&mut self) {
        let pos = self.get_cursor_position();
        let (x, y) = (pos.x, pos.y);
        let (num_lines, chars_left_on_line, next_line_len) = {
            let lines = self.get_editor_lines();
            let next_line_len = if y + 1 < lines.len() {
                lines[y+1].len()
            } else {
                0
            };
            (lines.len(), lines[y].len() - x, next_line_len)
        };
        let is_last_line = num_lines > 0 &&  y == num_lines - 1;
        if is_last_line {
            self.cursor_to_eof();
        } else {
            let this_line_len = x + chars_left_on_line;
            if x > next_line_len && next_line_len < this_line_len {
                self.cursor_index += chars_left_on_line + next_line_len - 1;
            } else {
                self.cursor_index +=  chars_left_on_line + x;
            }
        }
    }

    fn cursor_line_up(&mut self) {
        if self.cursor_index == 0 {
            return;
        }
        
        // The indices of newlines before the cursor position
        let newline_indices: Vec<usize> = self.piece_table.iter()
                                              .take(self.cursor_index)
                                              .enumerate()
                                              .filter(|&(_, ch)| ch == '\n')
                                              .map(|(index, _)| index)
                                              .collect();
        
        let num_newlines = newline_indices.len();
        if num_newlines == 0 {
            self.cursor_index = 0;
            return;
        } 
        
        let most_recent_newline = newline_indices.last().unwrap();
        let x = self.cursor_index - most_recent_newline - 1;
        if num_newlines == 1 {
            // If we're on the second line
            self.cursor_index = x;
        } else {
            let second_most_recent_newline = newline_indices[num_newlines - 2];
            if x > most_recent_newline - second_most_recent_newline {
                // If our x coord is greater than the length of the line above, go to end of above line
                self.cursor_index -= x + 1;
            } else {
                // If there's characters to move to directly above the cursor, move to char directly above
                self.cursor_index = second_most_recent_newline + x + 1;
            }
        }
    }

    fn cursor_start_next_word(&mut self) {
        let increment_by = self.piece_table.iter()
                                           .skip(self.cursor_index)
                                           .take_while(|ch| !ch.is_whitespace())
                                           .count() + 1;
                                           
        let last_index = self.get_file_length_in_chars() - 1;
        let indices_to_eof = last_index - self.cursor_index;
        self.cursor_index += cmp::min(increment_by, indices_to_eof);
    }

    // TODO: Extract logic from following 2 functions into a new function that takes
    // a Predicate argument (FnMut(Self::Item) -> Option<B>), and finds the first match
    // of the predicate before the cursor index. Do the same for the first match after
    // the cursor index.
    fn cursor_start_prev_word(&mut self) {
        let chars_to_take = if self.cursor_index > 0 { self.cursor_index - 1 } else { 0 };
        self.cursor_index = self.piece_table.iter()
                                            .take(chars_to_take)
                                            .enumerate()
                                            .filter(|&(_, ch)| ch.is_whitespace())
                                            .last()
                                            .map(|(whitespace_idx, _)| whitespace_idx + 1)
                                            .unwrap_or(0);
    }

    fn cursor_start_of_line(&mut self) {
        let chars_to_take = if self.cursor_index > 0 { self.cursor_index - 1 } else { 0 };
        self.piece_table.iter()
                        .take(chars_to_take)
                        .enumerate()
                        .filter(|&(_, ch)| ch == '\n')
                        .last()
                        .map(|(newline_idx, _)| newline_idx + 1)
                        .unwrap_or(0);
    }

    fn cursor_end_of_line(&mut self) {
        self.cursor_index += self.piece_table.iter()
                                             .skip(self.cursor_index)
                                             .take_while(|ch| *ch != '\n')
                                             .count();
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
