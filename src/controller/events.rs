use std::collections::HashMap;
use std::io::{stdin, stdout};

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Event;

use syntect::easy::HighlightLines;

use data::editor_state::{StateApi, EditorState, Mode};
use controller::input::{ModeInputHandler, NavigateModeInputHandler, CommandModeInputHandler, InsertModeInputHandler};
use view::terminal::*;

pub struct InputModeMultiplexer {
    mapping: HashMap<Mode, Box<ModeInputHandler>>
}

impl InputModeMultiplexer {
    pub fn new() -> InputModeMultiplexer {
        let mut mode_mappings: HashMap<Mode, Box<ModeInputHandler>> = HashMap::new();

        mode_mappings.insert(Mode::Navigate, Box::new(NavigateModeInputHandler::new()));
        mode_mappings.insert(Mode::Command, Box::new(CommandModeInputHandler::new()));
        mode_mappings.insert(Mode::Insert, Box::new(InsertModeInputHandler::new()));

        InputModeMultiplexer {
            mapping: mode_mappings
        }
    }

    pub fn get_command_buffer(&mut self, state: &EditorState) -> &Vec<char> {
        // Gets the input buffer for the currently active mode
        let mode_handler = self.mapping.get_mut(&state.get_mode()).unwrap();
        mode_handler.get_input_buffer()

    }

    pub fn do_action_for_input(&mut self, event: Event, state: &mut EditorState) -> &Vec<char> {
        // We get the correct handler for the node, and forward the input character on to that.
        // The handler deals with internal state management, command composition etc.
        let mode_handler = self.mapping.get_mut(&state.get_mode()).unwrap();
        mode_handler.handle_input(event, state)
    }

}

pub fn event_loop(highlighter: &mut HighlightLines, state: &mut EditorState) {

    
    let mut out = stdout().into_raw_mode().unwrap();
    clear_screen(&mut out);
    render(&mut out, highlighter, state);

    let mut input_mode_manager = InputModeMultiplexer::new();
    loop {
        let mut events = stdin().events();

        if let Some(event) = events.next() {
            let mode_input_buffer = input_mode_manager.do_action_for_input(event.unwrap(), state);
            state.mode_input_buffer = mode_input_buffer.clone();
        }
        render(&mut out, highlighter, state);
    }

}