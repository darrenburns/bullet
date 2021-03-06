use std::fmt;
use std::process::exit;

use termion::event::{Event, Key};

use controller::util::{repeater_chain_to_usize, repeat_state_op};
use controller::commands::{event_to_fn_alias, build_op_from_event};
use data::editor_state::{StateApi, EditorState, Mode};
use data::io::*;

#[derive(Clone, Debug)]
pub enum Action {
    Right,
    Left,
    Down,
    Up,
    StartNextWord,
    StartPrevWord,
    StartOfLine,
    EndOfLine,
    ToCommandMode,
    ExitEditor
}

#[derive(Clone, Debug)]
pub enum FnAlias {
    FindNext,
    NoOp
}

#[derive(Clone, Debug)]
pub enum FnArg {
    NoArg,
    Argument(char),
}

type Argument = char;
type MappedKey = char;
#[derive(Debug, Clone)]
pub struct Repeatable { 
    pub times: String, 
    pub expr: Option<ExecutableExpr> 
}
#[derive(Debug, Clone)]
pub enum ExecutableExpr {
    Operator ( Action ),
    Function ( FnAlias, FnArg )
}

// State machine used to validate navigation expressions, represents the LAST event seen (not what is next expected)
#[derive(Clone, Debug)]
pub enum ExprState {
    Waiting,  // There's nothing in the command buffer, can expect anything
    Function { repeatable: Repeatable },  // The input received means we're expecting the next input to be a suitable argument
    Repeater { repeatable: Repeatable },  // We've received a repeater (prefixing number), so we know the next state has to be another number, a function or a terminal character
    Execute { repeatable: Repeatable },  // e.g. 'w' to move to start of next word - leads to terminal state.
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_write = match *self {
            Action::Right => "Right",
            Action::Left => "Left",
            Action::Down => "Down",
            Action::Up => "Up",
            Action::StartNextWord => "StartNextWord",
            Action::StartPrevWord => "StartPrevWord",
            Action::StartOfLine => "StartOfLine",
            Action::EndOfLine => "EndOfLine",
            Action::ToCommandMode => "ToCommandMode",
            Action::ExitEditor => "EcitEditor"
        };

        write!(f, "{}", to_write)
    }
}


impl fmt::Display for FnArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FnArg::NoArg => write!(f, "_"),
            FnArg::Argument(arg) => write!(f, "{}", arg)
        }
    }
}

impl fmt::Display for FnAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_write = match *self {
            FnAlias::FindNext => "FindNext",
            FnAlias::NoOp => "NoOp"
        };
        write!(f, "{}", to_write)
    }
}

impl fmt::Display for ExecutableExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExecutableExpr::Operator( ref action) => 
                write!(f, "CursorTo ( {} )", &action),

            ExecutableExpr::Function( ref alias, ref arg) => 
                write!(f, "{} ( {} )", &alias, &arg)
        }
    }
}

impl fmt::Display for ExprState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExprState::Waiting => write!(f, "None"),
            ExprState::Repeater { ref repeatable } => 
                write!(f, "Repeat ( {}x, _ )", &repeatable.times),
            ExprState::Function { ref repeatable } => 
                write!(f, "Repeat ( {}x, {} )", &repeatable.times, &repeatable.clone().expr.unwrap()),
            ref expr @ _ => write!(f, "{}", expr)
        }
    }
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
                    // Waiting -> Repeater transition (We've received character 0-9)
                    Event::Key(Key::Char(c @ '0'...'9')) => {
                        move_to_state = ExprState::Repeater { 
                            repeatable: Repeatable { 
                                times: c.to_string(), 
                                expr: None,
                            }
                        };
                    }

                    // Either move to Function state if the input is a Function name, or straight to Execute
                    _ => {
                        match event_to_fn_alias(&event) {
                            // No function was found, so we don't expect any further input.
                            // We move to state Execute, which causes the actual execution of the ExecutableExpr
                            // we've built up.
                            FnAlias::NoOp => move_to_state = ExprState::Execute {
                                repeatable: Repeatable {
                                    times: String::from("1"),
                                    expr: build_op_from_event(&event)
                                }
                            },
                            // We've found a Function for this event, move into Function state,
                            // which will await the argument input
                            alias => move_to_state = ExprState::Function {
                                repeatable: Repeatable {
                                    times: String::from("1"),
                                    expr: Some(
                                        ExecutableExpr::Function(alias, FnArg::NoArg)
                                    )
                                }
                            },
                        }
                    }
                }
            },

            &ExprState::Repeater { ref repeatable } => {
                match event {
                    // Repeater -> Repeater transition
                    Event::Key(Key::Char(c @ '0'...'9')) => {
                        move_to_state = ExprState::Repeater { 
                            repeatable: Repeatable {
                                times: format!("{}{}", repeatable.times, c),
                                expr: None
                            }  
                        };
                     },
                    
                    // Repeater -> Function transition
                    _ => {
                        match event_to_fn_alias(&event) {
                            // No functions match, execute repeated Operation
                            FnAlias::NoOp => move_to_state = ExprState::Execute {
                                repeatable: Repeatable {
                                    times: repeatable.clone().times,
                                    expr: build_op_from_event(&event)
                                }
                            },

                            // A Function exists for the input char, enter Function stage
                            // with the appropriate FnAlias activated.
                            alias => move_to_state = ExprState::Function {
                                repeatable: Repeatable {
                                    times: repeatable.clone().times,
                                    expr: Some(
                                        ExecutableExpr::Function(event_to_fn_alias(&event), FnArg::NoArg)
                                    )
                                }
                            },
                        }
                    }
                }
            },

            // We're already in the Function stage, so this input is the arg
            &ExprState::Function { ref repeatable } => {
                // Grab the alias of the function from the Function stage
                let repeatable_clone = repeatable.clone();
                let alias = match repeatable_clone {
                    Repeatable {
                        times: _, 
                        expr: Some(ExecutableExpr::Function(alias, _))
                    } => alias,
                    _ => exit(0)
                };

                match event {
                    Event::Key(Key::Char(ch)) => {
                        // We've received an argument, so move to the Execute state, passing Argument through
                        move_to_state = ExprState::Execute {
                            repeatable: Repeatable {
                                times: repeatable_clone.times,
                                expr: Some(
                                    ExecutableExpr::Function(alias, FnArg::Argument(ch))
                                )
                            }
                        }
                    }
                    _ => move_to_state = ExprState::Waiting
                }
            }

            _ => move_to_state = ExprState::Waiting,
        }
        let mut finalised_state = move_to_state.clone();
        self.goto_state(state_api, move_to_state);

        // This'll deal only with termination (when the latest input has put us into the Execute state)
        if let &ExprState::Execute { ref repeatable } = self.get_state() {
            match repeatable {

                // Handle operators
                &Repeatable { ref times, expr: Some(ExecutableExpr::Operator(ref action)) } =>
                    match action {
                        &Action::Right =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::inc_cursor, 
                                state_api
                            ),
                        &Action::Left =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::dec_cursor, 
                                state_api
                            ),
                        &Action::Down =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::cursor_line_down, 
                                state_api
                            ),
                        &Action::Up =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::cursor_line_up, 
                                state_api
                            ),
                        &Action::StartNextWord =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::cursor_start_next_word, 
                                state_api
                            ),
                        &Action::StartPrevWord =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::cursor_start_prev_word, 
                                state_api
                            ),
                        &Action::StartOfLine =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::cursor_start_of_line, 
                                state_api
                            ),
                        &Action::EndOfLine =>
                            repeat_state_op(
                                &repeater_chain_to_usize(times), 
                                &StateApi::cursor_end_of_line, 
                                state_api
                            ),
                        &Action::ToCommandMode => state_api.set_mode(Mode::Command), 
                        &Action::ExitEditor => exit(0), 
                        _ => ()

                    }
                
                &Repeatable { ref times, expr: Some(ExecutableExpr::Function(ref alias, FnArg::Argument(ref arg))) } =>
                    match alias {
                        &FnAlias::NoOp => (),
                        
                        // TODO: FindNext implementation is just for testing purposes atm.
                        &FnAlias::FindNext => {
                            repeat_state_op(
                                &(arg.to_string().parse::<usize>().unwrap_or(1) * 
                                    repeater_chain_to_usize(times)),
                                &StateApi::cursor_line_down,
                                state_api
                            );
                        }
                    },
                _ => ()
            }
            finalised_state = ExprState::Waiting;
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

