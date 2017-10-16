extern crate rustty;

mod data;
mod view;

use rustty::Terminal;

use std::io::prelude::*;
use std::fs::File;


fn main() {    
    let mut term = view::terminal::create_terminal();
    
    let mut file = File::open("test.txt")
                        .expect("Unable to open file");

    let piece_table = data::piece_table::PieceTable::new(&mut file);

    loop {
        view::terminal::draw_terminal(&mut term);

    }
}
