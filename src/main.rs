extern crate rustty;

mod data;
mod view;

use rustty::Terminal;

fn main() {    
    let mut term = view::sandbox::create_terminal();
    loop {
        view::sandbox::draw_terminal(&mut term);
        term.swap_buffers().unwrap();
    }

}
