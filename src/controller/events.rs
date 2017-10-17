use std::process::exit;
use std::time::Duration;
use std::collections::HashMap;

use rustty::{Terminal, Event};

use data::editor_state::{StateApi, EditorState};
use view::terminal::*;

struct InputActionMapping {
    mapping: HashMap<char, Box<BulletCommand>>   
}

impl InputActionMapping {
    pub fn new() -> InputActionMapping {
        InputActionMapping {
            mapping: HashMap::new()
        }
    }

    pub fn do_action_for_input(&self, input_char: char, state: &mut EditorState, term: &mut Terminal) {
        if let Some(command) = self.mapping.get(&input_char) {
            command.execute(state, term)
        }
    }

    pub fn add_mapping(&mut self, ch: char, command: Box<BulletCommand>) {
        self.mapping.insert(ch, command);
    }

}

pub struct CommandError {
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

struct AnotherCommand {}
impl BulletCommand for AnotherCommand {
    fn execute(&self, state: &mut EditorState, terminal: &mut Terminal) {

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

fn register_input_action_mapping() -> InputActionMapping {
    let mut input_action_mapping = InputActionMapping::new();
    input_action_mapping.add_mapping('q', Box::new(QuitCommand {}));
    input_action_mapping.add_mapping('w', Box::new(AnotherCommand {}));
    input_action_mapping
}
