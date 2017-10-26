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
            // Basic directional movement
            'h' => state_api.dec_cursor(1),
            'l' => state_api.inc_cursor(1),
            'j' => state_api.cursor_line_down(),
            'k' => state_api.cursor_line_up(),

            // Content-aware movement
            'w' => state_api.cursor_start_next_word(),
            'b' => state_api.cursor_start_prev_word(),
            '$' => state_api.cursor_end_of_line(),

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

