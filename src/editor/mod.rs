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
        execute!(self.stdout, crossterm::event::EnableMouseCapture).unwrap();
        // 清空屏幕
        render::render(&mut self.stdout, 1, content);
        Ok(())
    }
}

impl Editor {
    pub(crate) fn move_up(&mut self) {
        // 获取光标位置
        let (x, y) = cursor::position().unwrap();
        // 判断位置是否在第一行
        if y == 0 {
            // 判断当前内容是否为第一行，将内容向上滚动一行
            if x == 0 {
                self.scroll_up();
            }
            return;
        }
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
