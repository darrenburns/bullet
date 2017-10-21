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
            'j' => cursor_line_down(1, state_api),
            ';' => state_api.set_mode(Mode::Command),
            'q' => {
                exit(0);
            }
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

fn cursor_line_down(num_lines: usize, state: &mut EditorState) {
    let num_lines = state.get_editor_lines().len();
    
    // If we're on the last line, go to the end of the file
    let pos = state.get_cursor_position();
    let x = pos.x;
    let y = pos.y;
    if num_lines > 0 &&  y == num_lines - 1 {
        let file_len = state.get_file_length_in_chars() - 1;
        state.cursor_index = file_len - 1;
    } else {
        // Otherwise, note how far along the current line we are
        
        // Move to the start of the line below (just after the newline)
        
        // Add on how far along the line above we were to the cursor index, 
        // or to the end if this line is shorter

        // Repeat num_lines times???
    }

}
