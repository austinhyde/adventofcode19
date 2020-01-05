use crossterm::{
    cursor,
    event::{poll, read, Event, KeyEvent},
    execute, queue, terminal,
};
use intcode::Word;
use std::io::{stdout, Stdout, Write};

pub struct Screen {
    stdout: Stdout,
}

impl Screen {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }
    pub fn clear(&mut self) {
        self.moveto(0, 0);
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    }
    pub fn hide(&mut self) {
        execute!(self.stdout, cursor::Hide, cursor::DisableBlinking).unwrap();
    }
    pub fn moveto(&mut self, x: Word, y: Word) {
        queue!(self.stdout, cursor::MoveTo(x as u16, y as u16)).unwrap();
    }
    pub fn flush(&mut self) {
        self.stdout.flush().unwrap()
    }
    pub fn put(&mut self, x: Word, y: Word, s: &str) {
        self.moveto(x, y);
        self.write(s);
        self.flush();
    }
    pub fn write(&mut self, s: &str) {
        self.stdout.write(s.as_bytes()).unwrap();
    }
    pub fn key_pressed(&mut self, timeout: std::time::Duration) -> Option<KeyEvent> {
        if poll(timeout).unwrap() {
            match read().unwrap() {
                Event::Key(kev) => Some(kev),
                _ => None,
            }
        } else {
            None
        }
    }
}
