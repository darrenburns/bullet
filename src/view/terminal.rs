use rustty::{CellAccessor, Terminal, Color, HasSize};
use rustty::ui::Widget;

pub trait Drawable {
    fn draw<T>(&self, canvas: T) where T: Canvas;
}

// Use Canvas trait rather than Terminal to stay implementation agnostic
pub trait Canvas {}
impl Canvas for Terminal {}

pub fn create_terminal() -> Terminal {
    return Terminal::new().unwrap();
}

pub fn draw_terminal(term: &mut Terminal) {
    draw_gutter(term);
    term.swap_buffers().unwrap();
}

fn draw_gutter(term: &mut Terminal) {
    let mut gutter = Widget::new(1, term.size().1);
    {
        let mut cells = gutter.cellvec_mut();
        for cell in cells.iter_mut() {
            cell.set_ch('X')
                .set_fg(Color::Black)
                .set_bg(Color::Magenta);
        }
    }
    gutter.draw_into(term);
}