use std::io::stdout;

use crossterm::{cursor, execute, terminal};
use unicode_segmentation::UnicodeSegmentation;

use crate::editor::render::Render;
use crate::editor::Editor;

impl Editor {
    pub(crate) fn reset_cursor() {
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
    pub(crate) fn move_up(&mut self) {
        let mut stdout = stdout();
        // 获取光标位置
        let (_, cursor_height) = cursor::position().unwrap();
        // 判断位置是否在第一行，且当前显示的内容不可以为第一行
        if cursor_height == 0 && self.start_line > 0 {
            self.scroll_down();
            Render::render_scroll_down(&mut stdout, self);
            return;
        }
        execute!(&mut stdout, cursor::MoveUp(1)).unwrap();
        Render::check_line_end(&mut stdout, self);
    }

    pub(crate) fn move_down(&mut self) {
        let mut stdout = stdout();
        // 获取光标位置
        let (_, cursor_height) = cursor::position().unwrap();
        // 判断位置是否在最后一行，且当前显示的内容还存在剩余行
        if cursor_height as usize == self.content_height {
            if (self.start_line + self.content_height) < self.file_content.content.len() - 1 {
                self.scroll_up();
                Render::render_scroll_up(&mut stdout, self);
            }
            return;
        }
        execute!(stdout, cursor::MoveDown(1)).unwrap();
        Render::check_line_end(&mut stdout, self);
    }

    pub(crate) fn move_left(&mut self) {
        let mut stdout = stdout();
        // 获取光标位置
        let (cursor_width, _) = cursor::position().unwrap();
        let cursor_width = cursor_width as usize;
        if cursor_width <= self.line_number_len + 1 {
            return;
        }
        execute!(stdout, cursor::MoveLeft(1)).unwrap();
    }
    pub(crate) fn move_right(&mut self) {
        let mut stdout = stdout();
        // 获取光标位置
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let current_line_number = self.start_line + cursor_height as usize;
        let current_line_info = match self.file_content.content.get(current_line_number) {
            Some(line_info) => line_info,
            None => {
                return;
            }
        };
        // 检查行字符串末尾是否为换行符
        let mut current_line_text_len = current_line_info.text.graphemes(true).count();
        if current_line_info.text.ends_with("\r\n") {
            current_line_text_len -= 1;
        }
        if cursor_width as usize > self.line_number_len + current_line_text_len {
            return;
        }
        execute!(stdout, cursor::MoveRight(1)).unwrap();
    }
    pub(crate) fn scroll_up(&mut self) {
        let mut stdout = stdout();
        execute!(stdout, terminal::ScrollUp(1)).unwrap();
    }
    pub(crate) fn scroll_down(&mut self) {
        let mut stdout = stdout();
        execute!(stdout, terminal::ScrollDown(1)).unwrap();
    }
}
