use std::{fs, io};

// const SCROLL_FACTOR: usize = 2;
// const TAB_LENGTH: usize = 4;

pub struct EditorView {
    pub line_offset: usize,
    pub text_start_pos: u16,
    pub rope: ropey::Rope,
}

impl EditorView {
    pub fn new() -> EditorView {
        EditorView {
            line_offset: 0,
            text_start_pos: 0,
            rope: ropey::Rope::from_str("Hello, world. \n\n\n"),
        }
    }
    pub fn load_file(filename: &str) -> EditorView {
        let file = fs::File::open(filename).unwrap();
        let buf_reader = io::BufReader::new(file);
        let rope = ropey::Rope::from_reader(buf_reader).unwrap();
        let total_lines = rope.len_lines();
        EditorView {
            line_offset: 0,
            text_start_pos: (total_lines as f64).log10() as u16 + 1,
            rope,
        }
    }
}

impl super::Widget for EditorView {
    fn render(&self, screen: &super::screen::Screen) {
        paint_lines(&self, screen);
    }
}

fn paint_lines(editor_view: &EditorView, screen: &super::screen::Screen) {
    for i in 0..20 {
        if i < editor_view.rope.len_lines() {
            let line = editor_view.rope.line(i).as_str().unwrap();
            screen.draw(
                0,
                i as u16,
                &format!(
                    "{}{:>width$} {}{}",
                    termion::color::Bg(termion::color::LightBlack),
                    i + 1,
                    termion::color::Bg(termion::color::Black),
                    line,
                    width = editor_view.text_start_pos as usize
                ),
            );
        }
    }
}
