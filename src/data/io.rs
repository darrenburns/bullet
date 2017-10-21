use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;

use data::editor_state::{StateApi, EditorState};


pub fn write_file(state: &mut EditorState) {
    let mut buffer = BufWriter::new(
        File::create("saved_file.txt")
             .expect("Unable to create file")
    );

    for line in state.get_editor_lines().iter() {
        buffer.write((*line).as_bytes());
        buffer.write("\n".as_bytes());  // Naive.
    }
} 