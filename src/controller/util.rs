use data::editor_state::EditorState;
use controller::input::FnAlias;


pub fn repeater_chain_to_usize(repeater_chain: &str) -> usize {
    repeater_chain.parse::<usize>().unwrap_or(1)
}

pub fn repeat_state_op(times: &usize, func: &Fn(&mut EditorState) -> (), state: &mut EditorState) {
    for _ in 0..*times {
        func(state);
    }
}
