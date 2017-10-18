use data::editor_state::{StateApi, EditorState, Mode};


pub trait ModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState);
}

pub struct NavigateModeInputHandler {}
impl ModeInputHandler for NavigateModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState) {
        match input_char {
            ';' => state_api.set_mode(Mode::Command),
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
}
impl ModeInputHandler for CommandModeInputHandler {
    fn handle_input(&mut self, input_char: char, state_api: &mut EditorState) {
        // Handle Command mode input - add chars to buffer until enter is pressed.
        // When enter is pressed, execute buffered commands and clear buffer.
        // Return to navigate mode.
        match input_char {
            '\r' => (),  // TODO: Actually process the command string
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
