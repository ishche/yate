use std::{fs, io};

// const SCROLL_FACTOR: usize = 2;
// const TAB_LENGTH: usize = 4;

pub struct EditorView {
    pub line_offset: usize,
    pub rope: ropey::Rope,
}

impl EditorView {
    pub fn new() -> EditorView {
        EditorView {
            line_offset: 0,
            rope: ropey::Rope::from_str("Hello, world. \n\n\n"),
        }
    }
    pub fn load_file(filename: &str) -> EditorView {
        let file = fs::File::open(filename).unwrap();
        let buf_reader = io::BufReader::new(file);
        EditorView {
            line_offset: 0,
            rope: ropey::Rope::from_reader(buf_reader).unwrap(),
        }
    }
}

impl super::Widget for EditorView {
    fn render(&self, screen: &super::screen::Screen) {
        paint_lines(&self, screen);
    }
}

fn paint_lines(editor_view: &EditorView, screen: &super::screen::Screen) {
    let total_lines = editor_view.rope.len_lines();
    let shift = (total_lines as f64).log10() as usize + 1;
    for i in 0..20 {
        if i < editor_view.rope.len_lines() {
            let line = editor_view.rope.line(i).as_str().unwrap();
            screen.draw(0, i as u16, &format!("{:>width$} {}", i + 1, line, width = shift));
        }
    }
}
