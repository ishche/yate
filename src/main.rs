extern crate termion;
use std::io::{stdin, Write};
use termion::event::Key;
use termion::input::TermRead;

mod screen;

fn main() {
    let stdin = stdin();
    let stdin = stdin.lock();
    let mut screen = screen::Screen::new();

    let (x,y) = screen.cursor_pos;
    
    write!(
        screen.out.borrow_mut(),
        "{}{}{}'q' will exit.{}{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(5, 5),
        termion::style::Bold,
        termion::style::Reset,
        termion::cursor::Goto(20, 10),
        termion::cursor::Show,
        termion::cursor::BlinkingBar,
        termion::cursor::Goto(x, y)
    )
    .unwrap();
    screen.out.borrow_mut().flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => screen.cursor_shift(-1, 0),
            Key::Right => screen.cursor_shift(1, 0),
            Key::Up => screen.cursor_shift(0, -1),
            Key::Down => screen.cursor_shift(0, 1),
            Key::Backspace => println!("×"),
            _ => {}
        }
        let (x,y) = screen.cursor_pos;
        write!(screen.out.borrow_mut(), "{}", termion::cursor::Goto(x, y));
        screen.out.borrow_mut().flush().unwrap();
    }
}
