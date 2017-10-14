use rustty::{Terminal, Cell, Color, HasSize, CellAccessor};


use rustty::ui::{Painter, Dialog, DialogResult, Alignable, HorizontalAlign, VerticalAlign};

pub fn create_terminal() -> Terminal {
    return Terminal::new().unwrap();
}

pub fn draw_terminal(term: &mut Terminal) {
    let mut main_dialog = create_maindlg();

    main_dialog.window_mut().align(
        term, 
        HorizontalAlign::Middle, 
        VerticalAlign::Middle,
        0,
    );

    main_dialog.window().draw_into(term);
}

fn create_maindlg() -> Dialog {
    let mut maindlg = Dialog::new(60, 10);
    let s = "Hello! This is a showcase of the ui module!";
    let x = maindlg.window().halign_line(s, HorizontalAlign::Middle, 1);
    maindlg.window_mut().printline(x, 2, s);
    maindlg.add_button("Foo", 'f', DialogResult::Custom(1));
    maindlg.add_button("Bar", 'b', DialogResult::Custom(2));
    maindlg.add_button("Quit", 'q', DialogResult::Ok);
    maindlg.draw_buttons();
    maindlg.window_mut().draw_box();
    maindlg
}