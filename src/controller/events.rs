use std::process::exit;
use std::time::Duration;
use std::collections::HashMap;

use rustty::{Terminal, Event};

use data::editor_state::{StateApi, EditorState, Mode};
use controller::input::{ModeInputHandler, NavigateModeInputHandler, CommandModeInputHandler, InsertModeInputHandler};
use view::terminal::*;

struct InputModeMultiplexer {
    mapping: HashMap<Mode, Box<ModeInputHandler>>
}

impl InputModeMultiplexer {
    pub fn new() -> InputModeMultiplexer {
        let mut mode_mappings: HashMap<Mode, Box<ModeInputHandler>> = HashMap::new();

        mode_mappings.insert(Mode::Navigate, Box::new(NavigateModeInputHandler{}));
        mode_mappings.insert(Mode::Command, Box::new(CommandModeInputHandler::new()));
        mode_mappings.insert(Mode::Insert, Box::new(InsertModeInputHandler{}));

        InputModeMultiplexer {
            mapping: mode_mappings
        }
    }

    pub fn do_action_for_input(&self, input_char: char, state: &mut EditorState, term: &mut Terminal) {
        // We get the correct handler for the node, and forward the input character on to that.
        // The handler deals with internal state management, command composition etc.
        let mode_handler = self.mapping.get(&state.get_mode()).unwrap();
        mode_handler.handle_input(input_char, state);
    }

}

struct CommandError {
    message: String,
}

trait BulletCommand {
    fn execute(&self, state: &mut EditorState, terminal: &mut Terminal);
}

struct QuitCommand {}
impl BulletCommand for QuitCommand {
    fn execute(&self, state: &mut EditorState, terminal: &mut Terminal) {
        clear_and_draw_terminal(terminal);
        exit(0);
    }
}

struct WriteCommand {}
impl BulletCommand for WriteCommand {
    fn execute(&self, state: &mut EditorState, terminal: &mut Terminal) {

    }
}

struct CommandModeBegin {
    command_buffer: Vec<char>
}
impl CommandModeBegin {
    fn new() -> Self {
        CommandModeBegin {
            command_buffer: vec![]
        }
    }
}
impl BulletCommand for CommandModeBegin {
    fn execute(&self, state: &mut EditorState, terminal: &mut Terminal) {
        state.set_mode(Mode::Command);
    }
}

pub fn event_loop(term: &mut Terminal, state: &mut EditorState) {
    let action_map = register_input_action_mapping();
    loop {
        if let Some(Event::Key(input_ch)) = term.get_event(Duration::new(0, 0)).unwrap() {
            action_map.do_action_for_input(input_ch, state, term);
        }

        let editor_lines = state.get_editor_lines();
        draw_cursor(term);
        draw_terminal(term, editor_lines, &state);
    }
}

fn register_input_action_mapping() -> InputModeMultiplexer {
    let mut input_action_mapping = InputModeMultiplexer::new();

    // input_action_mapping.add_mapping(Mode::Command, 'q', Box::new(QuitCommand {}));
    // input_action_mapping.add_mapping(Mode::Command, 'w', Box::new(WriteCommand {}));

    // input_action_mapping.add_mapping(Mode::Navigate, ';', Box::new(CommandModeBegin::new()));

    input_action_mapping
}
