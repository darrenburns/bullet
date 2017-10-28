use std::cmp;
use std::process::exit;

use termion::event::{Event, Key};
use termion::clear;

use data::editor_state::{StateApi, EditorState, Mode};
use data::io::*;

pub enum CommandAtom {
    Repeater(usize),
    Parameterised(char),
}

pub trait ModeInputHandler {
    // Returns the input buffer to inform client that command may not have been executed if buffer contains chars
    fn handle_input(&mut self, event: Event, state_api: &mut EditorState) -> &Vec<char>;
    fn get_input_buffer(&self) -> &Vec<char>;
    fn push_input(&mut self, ch: char);
}

pub struct NavigateModeInputHandler {
    command_buffer: Vec<char>
}
impl NavigateModeInputHandler {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![]
        }
    }

    fn repeatable(&mut self, func: &Fn(&mut EditorState) -> (), state: &mut EditorState) {
        let input_as_string: String = self.command_buffer.iter().collect();
        let iterations = input_as_string.parse::<usize>().unwrap_or(1);
        for _ in 0..iterations {
            func(state);
        }
        self.command_buffer.clear();
    }

}
impl ModeInputHandler for NavigateModeInputHandler {

    fn handle_input(&mut self, event: Event, state_api: &mut EditorState) -> &Vec<char> {
        match event {
            // Repeaters - commands prefixed with any positive number N are repeated N times
            Event::Key(Key::Char(c @ '0'...'9')) => self.push_input(c),

            // Parametrised - navigation commands that take arguments and thus require further input

            // Basic directional movement
            Event::Key(Key::Char('h')) | Event::Key(Key::Left) => self.repeatable(&StateApi::dec_cursor, state_api),
            Event::Key(Key::Char('l')) | Event::Key(Key::Right) => self.repeatable(&StateApi::inc_cursor, state_api),
            Event::Key(Key::Char('j')) | Event::Key(Key::Down) => self.repeatable(&StateApi::cursor_line_down, state_api),
            Event::Key(Key::Char('k')) | Event::Key(Key::Up) => self.repeatable(&StateApi::cursor_line_up, state_api),

            // Content-aware movement
            Event::Key(Key::Char('w')) => self.repeatable(&StateApi::cursor_start_next_word, state_api),
            Event::Key(Key::Char('b')) => self.repeatable(&StateApi::cursor_start_prev_word, state_api),
            Event::Key(Key::Char('$')) => state_api.cursor_end_of_line(),

            Event::Key(Key::Char(';')) => state_api.set_mode(Mode::Command),
            Event::Key(Key::Char('q')) => exit(0),
            _ => (),
        }
        self.get_input_buffer()

        // Handle input in navigation mode here.
        // Early iterations will not require a buffer.
        // Later iterations allowing for composition of navigation commands will.
    }

    fn get_input_buffer(&self) -> &Vec<char> {
        &self.command_buffer
    }

    fn push_input(&mut self, ch: char) {
        self.command_buffer.push(ch);
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
    fn handle_input(&mut self, event: Event, state_api: &mut EditorState) -> &Vec<char> {
        // Handle Command mode input - add chars to buffer until enter is pressed.
        // When enter is pressed, execute buffered commands and clear buffer.
        // Return to navigate mode.
        match event {
            Event::Key(Key::Char('\n')) => {
                self.process_command_buffer(state_api);
                state_api.set_mode(Mode::Navigate);
            },
            Event::Key(Key::Char(ch)) => self.command_buffer.push(ch),
            _ => ()
        }
        self.get_input_buffer()
    }

    fn get_input_buffer(&self) -> &Vec<char> {
        &self.command_buffer
    }

    fn push_input(&mut self, ch: char) {
        self.command_buffer.push(ch);
    }
}

pub struct InsertModeInputHandler {
    command_buffer: Vec<char>
}
impl InsertModeInputHandler {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![]
        }
    }
}
impl ModeInputHandler for InsertModeInputHandler {
    fn handle_input(&mut self, event: Event, state_api: &mut EditorState) -> &Vec<char> {
        // Handle input in insertion mode. Will need reference to the StateApi to 
        // update the editor state.
        self.get_input_buffer()
    }

    fn get_input_buffer(&self) -> &Vec<char> {
        &self.command_buffer
    }

    fn push_input(&mut self, ch: char) {
        
    }
}

