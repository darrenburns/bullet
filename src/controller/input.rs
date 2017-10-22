use std::cmp;
use std::process::exit;

use data::editor_state::{StateApi, EditorState, Mode};
use data::io::*;


pub trait ModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState);
}

pub struct NavigateModeInputHandler {}
impl ModeInputHandler for NavigateModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState) {
        match input_char {
            'h' => dec_cursor(1, state_api),
            'l' => inc_cursor(1, state_api),
            'j' => cursor_line_down(state_api),
            'k' => cursor_line_up(state_api),
            ';' => state_api.set_mode(Mode::Command),
            'q' => exit(0),
            _ => (),
        }
        // Handle input in navigation mode here.
        // Early iterations will not require a buffer.
        // Later iterations allowing for composition of navigation commands will.
    }
}

pub struct CommandModeInputHandler {
    command_buffer: Vec<char>
}
impl CommandModeInputHandler {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![]
        }
    }

    fn process_command_buffer(&mut self, state: &mut EditorState) {
        for cmd_char in self.command_buffer.iter() {
            match cmd_char {
                &'w' => write_file(state),
                &'q' => exit(0),
                _ => ()
            }
        }
        self.command_buffer.clear();
    }
}
impl ModeInputHandler for CommandModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState) {
        // Handle Command mode input - add chars to buffer until enter is pressed.
        // When enter is pressed, execute buffered commands and clear buffer.
        // Return to navigate mode.
        match input_char {
            '\r' => {
                println!("{:?}", self.command_buffer);
                self.process_command_buffer(state_api);
                state_api.set_mode(Mode::Navigate);
            },
            _ => self.command_buffer.push(input_char),
        }
    }
}

pub struct InsertModeInputHandler {}
impl ModeInputHandler for InsertModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState) {
        // Handle input in insertion mode. Will need reference to the StateApi to 
        // update the editor state.
    }
}

fn inc_cursor(inc_by: usize, state: &mut EditorState) {
    if state.cursor_index < state.get_file_length_in_chars() - 2 {
        state.cursor_index += inc_by;
    }
}

fn dec_cursor(dec_by: usize, state: &mut EditorState) {
    if state.cursor_index >= dec_by {
        state.cursor_index -= dec_by;
    }
}


// TODO: Move these into the state API - keep the controller layer as minimal as possible.
fn cursor_line_down(state: &mut EditorState) {
    let pos = state.get_cursor_position();
    let (x, y) = (pos.x, pos.y);
    let (num_lines, chars_left_on_line, next_line_len) = {
        let lines = state.get_editor_lines();
        let next_line_len = if y + 1 < lines.len() {
            lines[y+1].len() + 1
        } else {
            0
        };
        (lines.len(), lines[y].len() - x + 1, next_line_len)
    };
    let is_last_line = num_lines > 0 &&  y == num_lines - 1;
    if is_last_line {
        state.cursor_to_eof();
    } else {
        let this_line_len = x + chars_left_on_line;
        if x > next_line_len && next_line_len < this_line_len {
            state.cursor_index += chars_left_on_line + next_line_len - 1;
        } else {
            state.cursor_index +=  chars_left_on_line + x;
        }
    }
}

fn cursor_line_up(state: &mut EditorState) {
    let pos = state.get_cursor_position();
    let (x, y) = (pos.x, pos.y);
    if y == 0 {
        state.set_cursor_index(0);
    } else {
        let (prev_line_len, this_line_len) = {
            let lines = state.get_editor_lines();
            (lines[y-1].len() + 1, lines[y].len() + 1)
        };
        if prev_line_len < x {
            state.cursor_index -= x;
            state.cursor_index -= 1;
        } else {
            state.cursor_index -= x;
            state.cursor_index -= prev_line_len - x;
        }
    }
}
