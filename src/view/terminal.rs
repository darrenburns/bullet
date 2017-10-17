use std::cmp;
use std::fmt;

use rustty::{Cell, CellAccessor, Terminal, Color, HasSize, Attr};
use rustty::ui::{Widget, Painter};

use data::piece_table::PieceTable;

const GUTTER_WIDTH: usize = 3;
const GUTTER_PADDING: usize = 1;

pub trait Drawable {
    fn draw(&self, canvas: Terminal);
}

// Use Canvas trait rather than Terminal to stay implementation agnostic
pub trait Canvas {}
impl Canvas for Terminal {}

pub fn create_terminal() -> Terminal {
    return Terminal::new().unwrap();
}

pub fn draw_terminal(term: &mut Terminal, lines: Vec<&str>) {
    draw_editor_window(term, lines);
    term.swap_buffers().unwrap();
}

pub fn draw_cursor(term: &mut Terminal) {
    term.set_cursor(GUTTER_WIDTH, 0).unwrap();
} 

// TODO: Encapsulate all editor state into an easily renderable struct
// Pass state via this object instead of a Vec<&str>
// The file is represented internally as a piece table, but presented
// to the terminal client as a vector of string slices.
fn draw_editor_window(term: &mut Terminal, lines: Vec<&str>) {
    let terminal_height = term.size().1;
    {
        let last_visible_line_index = cmp::min(lines.len(), terminal_height);
        let visible_lines = lines[..last_visible_line_index].into_iter();

        for (y, line) in visible_lines.enumerate() {
            let line_number = y + 1;  // TODO: Change it to y + 1 + scroll_offset when scrolling ready

            // Paint the gutter.
            let gutter_cell = Cell::with_style(Color::Black, Color::Red, Attr::BoldUnderline);
            let line_number_string = format!("{:>width$}", line_number.to_string(), width=GUTTER_WIDTH);
            term.printline_with_cell(0, y, line_number_string.as_str(), gutter_cell);

            // Paint the characters.
            for (ch_idx, ch) in line.chars().enumerate() {
                let screen_ch_idx = ch_idx + GUTTER_WIDTH + GUTTER_PADDING;
                term[(screen_ch_idx, y)].set_ch(ch);
            }

        }
    }

    // for (i, line) in lines.into_iter().enumerate() {
    // }
}