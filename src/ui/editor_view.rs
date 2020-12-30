// const SCROLL_FACTOR: usize = 2;
// const TAB_LENGTH: usize = 4;

pub struct EditorView {
    pub line_offset: usize
}

impl EditorView {
    pub fn new() -> EditorView {
        EditorView {line_offset: 0}
    }
}

impl super::Widget for EditorView {
    fn render(&self, screen: &super::screen::Screen) {
        paint_lines(screen);
    }
}

fn paint_lines(screen: &super::screen::Screen) {
    for i in 1..20 {
        screen.draw(0, i-1, &format!("{}", i));
    }    
}
