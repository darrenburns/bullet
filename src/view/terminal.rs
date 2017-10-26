use std::cmp;
use std::fmt::Write;

use rustty::{Cell, Terminal, Color, HasSize, Attr};
use rustty::ui::Painter;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

use data::editor_state::{StateApi, EditorState};

const GUTTER_WIDTH: usize = 3;
const GUTTER_RIGHT_MARGIN: usize = 1;

pub trait Drawable {
    fn draw(&self, canvas: Terminal);
}

// Use Canvas trait rather than Terminal to stay implementation agnostic
pub trait Canvas {}
impl Canvas for Terminal {}

pub fn create_terminal() -> Terminal {
    return Terminal::new().unwrap();
}

pub fn draw_terminal(term: &mut Terminal, highlighter: &mut HighlightLines, state: &EditorState) {
    draw_editor_window(term, highlighter, state);
    term.swap_buffers().unwrap();
}

pub fn draw_cursor(term: &mut Terminal, state: &EditorState) {
    let pos = state.get_cursor_position();
    term.set_cursor(pos.x + GUTTER_WIDTH + GUTTER_RIGHT_MARGIN, pos.y).unwrap();
}

pub fn clear_and_draw_terminal(term: &mut Terminal) {
    term.clear().unwrap();
    term.swap_buffers().unwrap();
}

// TODO: Encapsulate all editor state into an easily renderable struct
// Pass state via this object instead of a Vec<&str>
// The file is represented internally as a piece table, but presented
// to the terminal client as a vector of string slices.
fn draw_editor_window(term: &mut Terminal, line_highlighter: &mut HighlightLines, state: &EditorState) {
    let terminal_height = term.size().1;

    let lines = state.get_editor_lines();
    let last_visible_line_index = cmp::min(lines.len(), terminal_height);
    let visible_lines = lines[..last_visible_line_index].into_iter();
    let total_gutter_offset = GUTTER_WIDTH + GUTTER_RIGHT_MARGIN;

    for (y, line) in visible_lines.enumerate() {
        let line_number = y + 1;  // TODO: Change it to y + 1 + scroll_offset when scrolling ready

        // Paint the gutter.
        let gutter_cell = Cell::with_style(Color::Default, Color::Byte(0x00), Attr::Default);
        let line_number_string = format!("{:>width$} ", line_number.to_string(), width=GUTTER_WIDTH-1);
        term.printline_with_cell(0, y, line_number_string.as_str(), gutter_cell);

        let ranges: Vec<(Style, &str)> = line_highlighter.highlight(line);

        // if y == 0 {
        //     // println!("{:?}", &ranges[0]);
        //     term.set_cursor(4, 0);
        //     println!("\x1b[38;2;255;100;0mTRUECOLOR HELLO\x1b[0m");
        //     term.set_cursor(4, 1);
        //     println!("\x1b[38;2;105;250;ANOTHER HELLO!!!\x1b[0m");

        //     // term.printline(8, 0, line);
        // }
        
        let left_offset = GUTTER_WIDTH + GUTTER_RIGHT_MARGIN;
        let mut offset_in_line = 0;
        for (style, string) in ranges {
            let r: u8 = (style.foreground.r / 32) << 5;
            let g: u8 = (style.foreground.g / 32) << 2;
            let b: u8 = (style.foreground.b / 64);
            let eight_bit = r + g + b;
            let colour = Color::Byte(eight_bit);
            term.printline_with_cell(left_offset + offset_in_line, y, string, Cell::with_style(colour, Color::Default, Attr::Default));
            offset_in_line += string.len();
        } 
        
    }

    draw_status_line(term, state);
}

fn draw_status_line(term: &mut Terminal, state: &EditorState) {
    let terminal_width  = term.size().0;
    let terminal_height = term.size().1;

    let pos = state.get_cursor_position();
    let status_string = format!("{mode} - {x}, {y}, cursor idx = {idx}", mode=state.get_mode().to_string(), x=pos.x, y=pos.y, idx=state.cursor_index);
    term.printline_with_cell(
        0, 
        terminal_height - 1, 
        format!("{status:>width$}", status=status_string, width=terminal_width).as_str(),
        Cell::with_style(Color::Default, Color::Byte(0x00), Attr::Bold)
    );
}
