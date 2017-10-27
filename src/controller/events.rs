use std::collections::HashMap;
use std::io::{stdin, stdout};

use termion::raw::IntoRawMode;
use termion::input::TermRead;

use syntect::easy::HighlightLines;

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

    pub fn do_action_for_input(&mut self, input_char: char, state: &mut EditorState) {
        // We get the correct handler for the node, and forward the input character on to that.
        // The handler deals with internal state management, command composition etc.
        let mode_handler = self.mapping.get_mut(&state.get_mode()).unwrap();
        mode_handler.handle_input(input_char, state)
    }

}

struct CommandError {
    message: String,
}


pub fn event_loop(highlighter: &mut HighlightLines, state: &mut EditorState) {
    let mut action_map = register_input_action_mapping();
    
    let mut out = stdout().into_raw_mode().unwrap();
    clear_screen(&mut out);
    render(&mut out, highlighter, state);
    loop {
        let mut events = stdin().events();

        if let Some(event) = events.next() {
            break;  // Press any key to exit for now
        }
        render(&mut out, highlighter, state);
    }
    // loop {
    //     if let Some(event) = stdin.events().next() {
    //         let ev = event.unwrap();
    //     }
    //     let mut out = BufWriter::new(stdout());
    //     render(&mut out, highlighter, state);
    // }

    // for event in stdin.events() {
    //     println!("{}Red", color::Fg(color::Red));
    //     match event {
    //         _ => println!("Event - {:?}", event)
    //     // }
    //     // action_map.do_action_for_input(input_ch, state);

    //     // draw_cursor(state);
    //     // draw_terminal(highlighter, state);
    //     }
    // }


}

fn register_input_action_mapping() -> InputModeMultiplexer {
    InputModeMultiplexer::new()
}
