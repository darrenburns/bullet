use controller::input::*;

use termion::event::{Event, Key};

pub fn build_fn_from_event(event: &Event) -> Option<ExecutableExpr> {
    // We need to use the same matching logic for both the 
    // Waiting -> Function transition and the Repeater -> Function
    // transition.
    match event {
        &Event::Key(Key::Char('f')) => 
            Option::from(ExecutableExpr::Function (FnAlias::FindNext, FnArg::NoArg)),
        _ => None,
    }
}

pub fn build_op_from_event(event: &Event) -> Option<ExecutableExpr> {
    match *event {
        // Basic, directional navigation
        Event::Key(Key::Char('h')) | Event::Key(Key::Left) => 
            Option::from(ExecutableExpr::Operator(Action::Left)),
        Event::Key(Key::Char('l')) | Event::Key(Key::Right) => 
            Option::from(ExecutableExpr::Operator(Action::Right)),
        Event::Key(Key::Char('j')) | Event::Key(Key::Down) => 
            Option::from(ExecutableExpr::Operator(Action::Down)),
        Event::Key(Key::Char('k')) | Event::Key(Key::Up) => 
            Option::from(ExecutableExpr::Operator(Action::Up)),

        // Content-aware navigation
        Event::Key(Key::Char('w')) => 
            Option::from(ExecutableExpr::Operator(Action::StartNextWord)),
        Event::Key(Key::Char('b')) => 
            Option::from(ExecutableExpr::Operator(Action::StartPrevWord)),
        Event::Key(Key::Char('^')) => 
            Option::from(ExecutableExpr::Operator(Action::StartOfLine)),
        Event::Key(Key::Char('$')) => 
            Option::from(ExecutableExpr::Operator(Action::EndOfLine)),

        Event::Key(Key::Char(';')) => 
            Option::from(ExecutableExpr::Operator(Action::ToCommandMode)),
        Event::Key(Key::Char('q')) => 
            Option::from(ExecutableExpr::Operator(Action::ExitEditor)),


        _ => None,
    }
}
