mod render;

use std::io::Write;
use crossterm::{execute, terminal};
use crossterm::cursor;
use crate::files::reader::FileContent;

pub(crate) struct Editor {
    pub(super) stdout: std::io::Stdout,
}

impl Editor {
    pub(crate) fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
        }
    }
}

impl Editor {
    pub(crate) fn show_content(&mut self, content: &FileContent) -> Result<(), String> {
        // 清空屏幕
        render::render(&mut self.stdout, 1, content);
        Ok(())
    }
}

impl Editor {
    pub(crate) fn move_up(&mut self) {
        execute!(self.stdout, cursor::MoveUp(1)).unwrap();
    }
    pub(crate) fn move_down(&mut self) {
        execute!(self.stdout, cursor::MoveDown(1)).unwrap();
    }
    pub(crate) fn move_left(&mut self) {
        execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
    }
    pub(crate) fn move_right(&mut self) {
        execute!(self.stdout, cursor::MoveRight(1)).unwrap();
    }
    pub(crate) fn scroll_up(&mut self) {
        execute!(self.stdout, terminal::ScrollUp(1)).unwrap();
    }
    pub(crate) fn scroll_down(&mut self) {
        execute!(self.stdout, terminal::ScrollDown(1)).unwrap();
    }
}
