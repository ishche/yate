use std::cell::RefCell;
use std::convert::TryInto;
use std::io::*;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

pub struct Screen {
    pub cursor_pos: (u16, u16),
    size: (u16, u16),
    pub out: RefCell<RawTerminal<MouseTerminal<AlternateScreen<BufWriter<Stdout>>>>>,
}

impl Screen {
    pub fn new() -> Screen {
        let (w, h) = termion::terminal_size().unwrap();
        let out = RefCell::new(
            MouseTerminal::from(AlternateScreen::from(BufWriter::with_capacity(
                1 << 14,
                stdout(),
            )))
            .into_raw_mode()
            .unwrap(),
        );
        Screen {
            cursor_pos: (1, 1),
            size: (w.into(), h.into()),
            out,
        }
    }

    pub fn cursor_shift(&mut self, dx: i16, dy: i16) {
        let (x, y) = self.cursor_pos;
        let new_x: u16 = (x as i16 + dx).try_into().unwrap();
        let new_y: u16 = (y as i16 + dy).try_into().unwrap();
        self.cursor_pos = (new_x, new_y);
    }

    pub fn draw(&self, x: u16, y: u16, text: &str) {
        write!(
            self.out.borrow_mut(),
            "{}{}",
            termion::cursor::Goto(x + 1, y + 1),
            text
        )
        .unwrap();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        write!(
            self.out.borrow_mut(),
            "{}{}{}",
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset),
            termion::clear::All,
        )
        .unwrap();
        // self.show_cursor();
    }
}
