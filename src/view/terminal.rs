use std::cmp;

use std::io::Write;

use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::util::as_24_bit_terminal_escaped;

use termion::{clear, style, cursor, color};
use termion::terminal_size;

use data::editor_state::{StateApi, EditorState};

const GUTTER_WIDTH: usize = 4;
const GUTTER_RIGHT_MARGIN: usize = 2;


// TODO: Encapsulate all editor state into an easily renderable struct
// Pass state via this object instead of a Vec<&str>
// The file is represented internally as a piece table, but presented
// to the terminal client as a vector of string slices.

pub fn clear_screen<W: Write>(out: &mut W) {
    write!(out, "{}", clear::All);
}

pub fn render<W: Write>(out: &mut W, highlighter: &mut HighlightLines, state: &EditorState) {
    let (width, height) = terminal_size().unwrap();
    draw_gutter_and_lines(out, highlighter, state);
    draw_status_line(out, height as usize, width as usize, state);
    draw_cursor(out, state);
    out.flush();
}

fn draw_gutter_and_lines<W: Write>(out: &mut W, highlighter: &mut HighlightLines, state: &EditorState) {
    let lines = state.get_editor_lines();
    let last_visible_line_index = cmp::min(lines.len(), terminal_size().unwrap().1 as usize);
    let visible_lines = lines[..last_visible_line_index].into_iter();

    write!(out, "{}", cursor::Hide);

    for (y, line) in visible_lines.enumerate() {
        draw_gutter_for_line_number(out, y + 1);
        let ranges: Vec<(Style, &str)> = highlighter.highlight(line);
        let line = as_24_bit_terminal_escaped(&ranges[..], false);
        write!(out, "{}", line);
    }
    write!(out, "{}{}", clear::AfterCursor, cursor::Show);
}

fn draw_gutter_for_line_number<W: Write>(out: &mut W, line_number: usize) {
    let total_gutter_offset = GUTTER_WIDTH + GUTTER_RIGHT_MARGIN;
    write!(
        out,
        "{}{}{}{}{}{}{}",
        cursor::Goto(1, line_number as u16),
        clear::CurrentLine,
        color::Fg(color::Rgb(95, 110, 109)),
        color::Bg(color::Rgb(8, 31, 40)),
        format!("{:>width$} ", line_number, width=GUTTER_WIDTH - 1),
        style::Reset,
        cursor::Goto(total_gutter_offset as u16, line_number as u16)
    );
}

fn draw_status_line<W: Write>(out: &mut W, term_height: usize, term_width: usize, state: &EditorState) {
    //  Editing: {{file_name}} | Mode: Navigate                 Ln 66, Col 68 | Python

    let mode_input_buf: String = state.get_mode_input_buffer().into_iter().collect();
    let mode_text = format!("{} ({})", state.get_mode(), mode_input_buf);
    let left_side = format!(
        "Editing: {file_name} | Mode: {mode} | Modifier: {state}",
        file_name=state.get_active_file_name(),
        mode=mode_text,
        state=state.expression_state
    );

    let cursor_pos = state.get_cursor_position();
    let right_side = format!(
        "Ln {ln}, Col {col}, Idx {idx} | {file_type}",
        ln=cursor_pos.y + 1,
        col=cursor_pos.x + 1,
        idx=state.cursor_index,
        file_type = "Python"
    );

    // Make the right hand side take up the remaining width of the terminal
    let cols_remaining_after_left = term_width - left_side.len();
    write!(
        out,
        "{goto_bottom}{clear_line}{fg}{bg}{lhs}{rhs:>pad$}{reset}",
        goto_bottom=cursor::Goto(1, term_height as u16),
        clear_line=clear::CurrentLine,
        fg=color::Fg(color::Rgb(95, 110, 109)),
        bg=color::Bg(color::Rgb(8, 31, 40)),
        lhs=left_side,
        rhs=right_side,
        pad=cols_remaining_after_left,
        reset=style::Reset,
    );
}

fn draw_cursor<W: Write>(out: &mut W, state: &EditorState) {
    let cursor_position = state.get_cursor_position();
    let total_gutter_offset = GUTTER_WIDTH + GUTTER_RIGHT_MARGIN;
    write!(
        out,
        "{}",
        cursor::Goto((cursor_position.x + total_gutter_offset) as u16, cursor_position.y as u16 + 1),
    );
}
