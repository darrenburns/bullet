use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct PieceTable {
    pub original_file: String,
    pub add_buffer: String,
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
            add_buffer: String::new(),
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

    pub fn iter(&self) -> PieceTableIterator {
        PieceTableIterator {
            char_offset: 0, piece_index: 0, piece_table: self
        }
    }

}

pub struct PieceTableIterator<'a> {
    char_offset: usize,
    piece_index: usize,
    piece_table: &'a PieceTable
}

impl<'a> Iterator for PieceTableIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let pieces = self.piece_table.get_pieces();
        if self.piece_index == pieces.len() {
            return None;
        }

        let current_piece = &self.piece_table.get_pieces()[self.piece_index];

        let ch = match current_piece.source {
            Source::Orig => self.piece_table.original_file.chars().nth(self.char_offset),
            Source::Add => self.piece_table.add_buffer.chars().nth(self.char_offset)
        };

        self.char_offset += 1;
        if self.char_offset == current_piece.length {
            // finished with current piece, move on to the next one
            self.piece_index += 1;
            self.char_offset = 0;
        }
        
        ch
    }

}
