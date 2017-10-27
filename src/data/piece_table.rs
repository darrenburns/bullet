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

        let file_length = orig_buffer.chars().count();
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

    pub fn as_lines(&self) -> Vec<String> {
        // Since we're read for now, we'll just load in the
        // return the lines of the original file for now,
        // rather than from the piece table buffers.

        // I would like to do something a little more complex here -
        // return a Vec of characters wrapped in some Struct which
        // indicates the source buffer of the character. This can then
        // be read by the client, and we can display characters from
        // the original file buffer in a different colour from the 
        // characters from the add buffer.

        // Perhaps it would also make sense to move the logic for 
        // returning this struct into a dedicated DebugIterator for piece
        // tables, which returns wrapped chars as opposed to the standard
        // Iterator implementation.

        // It would also make sense to get rid of this method in general
        // and just give the front-end access to the Iterator/DebugIterator,
        // which it can use to display the content as required.

        // Note this comment from the syntect documentation example `syncat`:
        // We use read_line instead of `for line in highlighter.reader.lines()` because that
        // doesn't return strings with a `\n`, and including the `\n` gets us more robust highlighting.
        // See the documentation for `SyntaxSet::load_syntaxes`.
        // It also allows re-using the line buffer, which should be a tiny bit faster.

        // Maybe we return something which implements the Reader trait, and pass that
        // to syntect?
        self.original_file.lines()
            .map(|line| line.to_string() + "\n")
            .collect()
    }

    pub fn get_pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn iter(&self) -> PieceTableIterator {
        PieceTableIterator {
            back_offset: 0,
            char_offset: 0, 
            piece_index: 0,
            back_piece_index: 0, 
            piece_table: self,
        }
    } 

}

#[derive(Debug)]
pub struct PieceTableIterator<'a> {
    char_offset: usize,
    back_offset: usize,  // Backwards offset for DoubleEndedIterator, back_offset=0 means last char in piece
    piece_index: usize,
    back_piece_index: usize,
    piece_table: &'a PieceTable
}

impl<'a> PieceTableIterator<'a> {

    fn iterators_crossed(&self) -> bool {
        let current_piece = &self.piece_table.get_pieces()[self.piece_index];

        let back_offset_real_index = current_piece.length - self.back_offset - 1;

        self.char_offset >= back_offset_real_index &&
        self.back_piece_index == self.piece_index
    } 

}

impl<'a> Iterator for PieceTableIterator<'a> {

    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let pieces = self.piece_table.get_pieces();
        let current_piece = &pieces[self.piece_index];
        
        let is_final_char = self.piece_index == pieces.len() - 1 &&
                            self.char_offset == current_piece.length;

        if is_final_char || self.iterators_crossed() {
            return None;
        }

        let index_in_buffer = current_piece.start + self.char_offset;
        let ch = match current_piece.source {
            Source::Orig => self.piece_table.original_file.chars().nth(index_in_buffer),
            Source::Add => self.piece_table.add_buffer.chars().nth(index_in_buffer)
        };

        self.char_offset += 1;
        if self.char_offset == current_piece.length {
            self.piece_index += 1;
            self.char_offset = 0;
        }
        
        ch
    }

    fn count(self) -> usize {
        self.len()
    }

}

impl<'a> ExactSizeIterator for PieceTableIterator<'a> {

    fn len(&self) -> usize {
        let pieces = self.piece_table.get_pieces().iter();
        pieces.map(|p| p.length)
              .sum()
    }

}

impl<'a> DoubleEndedIterator for PieceTableIterator<'a> {

    fn next_back(&mut self) -> Option<Self::Item> {
        let pieces = self.piece_table.get_pieces();
        let num_pieces = pieces.len();
        let current_piece = &pieces[num_pieces - self.back_piece_index - 1];
        
        let is_first_char = self.back_piece_index == num_pieces - 1 &&
                            self.back_offset == current_piece.length - 1;

        if is_first_char || self.iterators_crossed() {
            return None;
        }

        let index_in_buffer = current_piece.start + current_piece.length - self.back_offset - 1;
        let ch = match current_piece.source {
            Source::Orig => self.piece_table.original_file.chars().nth(index_in_buffer),
            Source::Add => self.piece_table.add_buffer.chars().nth(index_in_buffer)
        };

        self.back_offset += 1;
        if self.back_offset == current_piece.length {
            self.back_piece_index += 1;
            self.back_offset = 0;
        }

        ch
    }

}
