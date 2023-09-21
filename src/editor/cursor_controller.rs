use crossterm::{cursor, execute, terminal};

use crate::editor::{render, Editor};

impl Editor {
    pub(crate) fn move_up(&mut self) {
        // 获取光标位置
        let (_, cursor_height) = cursor::position().unwrap();
        // 判断位置是否在第一行，且当前显示的内容不可以为第一行
        if cursor_height == 0 && self.start_line > 0 {
            self.scroll_down();
            render::render_scroll_down(self);
            return;
        }
        execute!(self.stdout, cursor::MoveUp(1)).unwrap();
    }
    pub(crate) fn move_down(&mut self) {
        // 获取光标位置
        let (_, cursor_height) = cursor::position().unwrap();
        // 判断位置是否在最后一行，且当前显示的内容还存在剩余行
        if cursor_height as usize == self.content_height {
            if (self.start_line + self.content_height) < self.file_content.content.len() - 1 {
                self.scroll_up();
                render::render_scroll_up(self);
            }
            return;
        }
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
