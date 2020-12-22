use crate::screen::Screen;

const SCROLL_FACTOR: usize = 2;
const TAB_LENGTH: usize = 4;

pub struct EditorView {
    line_offset: usize,
    screen: Screen,
}

impl EditorView {
    pub fn new(screen: Screen) -> EditorView {
        EditorView {line_offset: 0, screen}
    }

    pub fn paint_lines(&self) {
        
    }
}