use std::cmp;
use std::process::exit;

use termion::event::{Event, Key};
use termion::clear;

use controller::util::{repeater_chain_to_usize, repeat_state_op};
use data::editor_state::{StateApi, EditorState, Mode};
use data::io::*;


// State machine used to validate navigation expressions, represents the LAST event seen (not what is next expected)
#[derive(Clone, Debug)]
pub enum ExprState {
    Waiting,  // There's nothing in the command buffer, can expect anything
    UnaryFunction ( char ),  // The input received means we're expecting the next input to be a suitable argument
    Argument,  // Argument to a function - leads to terminal state.
    Repeater { times: String },  // We've received a repeater (prefixing number), so we know the next state has to be another number, a function or a terminal character
    Operator { iterations: usize },  // e.g. 'w' to move to start of next word - leads to terminal state.
}

pub trait ModeInputHandler {
    // Returns the input buffer to inform client that command may not have been executed if buffer contains chars
    fn handle_input(&mut self, event: Event, state_api: &mut EditorState) -> &Vec<char>;
    fn get_input_buffer(&self) -> &Vec<char>;
    fn push_input(&mut self, ch: char);
}

pub struct NavigateModeInputHandler {
    expression_state: ExprState,  
    command_buffer: Vec<char>
}
impl NavigateModeInputHandler {
    pub fn new() -> Self {
        Self {
            expression_state: ExprState::Waiting,
            command_buffer: vec![]
        }
    }

    fn goto_state(&mut self, editor_state: &mut EditorState, state: ExprState) {
        editor_state.expression_state = state.clone();
        self.expression_state = state;
    }

    fn get_state(&mut self) -> &ExprState {
        &self.expression_state
    }

}
impl ModeInputHandler for NavigateModeInputHandler {

    fn handle_input(&mut self, event: Event, state_api: &mut EditorState) -> &Vec<char> {
        
        let move_to_state;
        match self.get_state() {

            &ExprState::Waiting => {
                match event {
                    // Waiting -> Repeater transition
                    Event::Key(Key::Char(c @ '0'...'9')) => {
                        // self.push_input(c);
                        move_to_state = ExprState::Repeater {times: c.to_string()};
                    }
                    // Sometimes we'll move to the Function state rather than just defaulting to Operator
                    _ => move_to_state = ExprState::Operator { iterations: 1 }
                }
            },

            &ExprState::Repeater {ref times} => {
                match event {
                    // Repeater -> Repeater transition
                    Event::Key(Key::Char(c @ '0'...'9')) => {
                        move_to_state = ExprState::Repeater {times: format!("{}{}", times, c)};  // Keep appending Repeaters
                     } 
                    
                    // Repeater -> Operator transition
                    // Sometimes we'll move to the Function state rather than just defaulting to Operator
                    _ => move_to_state = ExprState::Operator { iterations: repeater_chain_to_usize(times) }
                }
            },


            &ExprState::UnaryFunction(fn_char) => {
                match event {
                    _ => move_to_state = ExprState::Argument
                }
            }

            // Waiting -> Operator transitions
 

            // // Waiting -> UnaryFunction transition
            
            // // Nothing yet!


            _ => move_to_state = ExprState::Operator { iterations: 1 },
        }

        let mut finalised_state = move_to_state.clone();
        self.goto_state(state_api, move_to_state);

        // This'll deal only with termination (Arg and Op), should we be in a termination state after this block.
        match self.get_state() {
             &ExprState::Operator { ref iterations } => {
                 match event {
                    Event::Key(Key::Char('h')) | Event::Key(Key::Left) => repeat_state_op(iterations, &StateApi::dec_cursor, state_api),
                    Event::Key(Key::Char('l')) | Event::Key(Key::Right) => repeat_state_op(iterations, &StateApi::inc_cursor, state_api),
                    Event::Key(Key::Char('j')) | Event::Key(Key::Down) => repeat_state_op(iterations, &StateApi::cursor_line_down, state_api),
                    Event::Key(Key::Char('k')) | Event::Key(Key::Up) => repeat_state_op(iterations, &StateApi::cursor_line_up, state_api),

                    Event::Key(Key::Char('w')) => repeat_state_op(iterations, &StateApi::cursor_start_next_word, state_api),
                    Event::Key(Key::Char('b')) => repeat_state_op(iterations, &StateApi::cursor_start_prev_word, state_api),
                    Event::Key(Key::Char('$')) => state_api.cursor_end_of_line(),

                    Event::Key(Key::Char(';')) => state_api.set_mode(Mode::Command),
                    Event::Key(Key::Char('q')) => exit(0),
                    _ => ()
                 }
                 finalised_state = ExprState::Waiting;
             },
             &ExprState::Argument => finalised_state = ExprState::Waiting,
             _ => (),  
             // All of the other cases are waiting for other input, 
             // and they've set their states in the first match block.
        }
        self.goto_state(state_api, finalised_state);
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

