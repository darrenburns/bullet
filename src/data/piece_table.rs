use std::io;

use std::io::prelude::*;
use std::fs::File;
use view::terminal::Drawable;

#[derive(Debug)]
pub struct PieceTable {
    original_file: String,
    pieces: Vec<Piece>,
}

#[derive(Debug)]
struct Piece {
    source: Source,
    start: usize,
    length: usize,
}

#[derive(Debug)]
enum Source {
    Orig,
    Add
}

impl PieceTable {

    pub fn new(from_file: &mut File) -> PieceTable {
        let mut orig_buffer = String::new();
        from_file.read_to_string(&mut orig_buffer)
                 .expect("Unable to read file contents!");

        let file_length = orig_buffer.len();
        PieceTable {
            original_file: orig_buffer,
            pieces: vec![
                Piece { 
                    source: Source::Orig,
                    start: 0,
                    length: file_length
                }
            ],
        }
    }

}

impl Drawable for PieceTable {
    fn draw<T>(&self, canvas: T) {
        
    }
}
