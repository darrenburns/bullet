use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct PieceTable {
    original_file: String,
    pieces: Vec<Piece>,
}

#[derive(Debug)]
pub struct Piece {
    source: Source,
    start: usize,
    pub length: usize,
}

#[derive(Debug, PartialEq)]
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

    pub fn as_lines(&self) -> Vec<&str> {
        // Since we're read for now, we'll just load in the
        // return the lines of the original file for now,
        // rather than from the piece table buffers.
        self.original_file.lines().collect()
    }

    pub fn get_pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

}
