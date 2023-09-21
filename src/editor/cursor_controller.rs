use crossterm::{cursor, execute, terminal};
use unicode_segmentation::UnicodeSegmentation;

use crate::editor::Editor;

impl Editor {
    pub(crate) fn move_up(&mut self) {
        // 获取光标位置
        let (_, cursor_height) = cursor::position().unwrap();
        // 判断位置是否在第一行，且当前显示的内容不可以为第一行
        if cursor_height == 0 && self.start_line > 0 {
            self.scroll_down();
            self.render_scroll_down();
            self.check_line_end();
            return;
        }
        execute!(self.stdout, cursor::MoveUp(1)).unwrap();
        self.check_line_end();
    }

    pub(crate) fn move_down(&mut self) {
        // 获取光标位置
        let (_, cursor_height) = cursor::position().unwrap();
        // 判断位置是否在最后一行，且当前显示的内容还存在剩余行
        if cursor_height as usize == self.content_height {
            if (self.start_line + self.content_height) < self.file_content.content.len() - 1 {
                self.scroll_up();
                self.render_scroll_up();
            }
            self.check_line_end();
            return;
        }
        execute!(self.stdout, cursor::MoveDown(1)).unwrap();
        self.check_line_end();
    }

    pub(crate) fn check_line_end(&mut self) -> bool {
        // 获取光标位置
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let current_line_number = self.start_line + cursor_height as usize;
        let current_line_info = match self.file_content.content.get(&current_line_number) {
            Some(line_info) => line_info,
            None => {
                execute!(self.stdout, cursor::MoveTo(0, cursor_height)).unwrap();
                return true;
            }
        };
        // 检查行字符串末尾是否为换行符
        let mut current_line_text_len = current_line_info.text.len();
        if current_line_info.text.ends_with('\n') {
            current_line_text_len -= 2;
        }
        let column_number_len = 2;
        let max_column_number = column_number_len + current_line_text_len;
        if cursor_width as usize >= max_column_number {
            execute!(
                self.stdout,
                cursor::MoveTo(max_column_number as u16, cursor_height)
            )
            .unwrap();
            return true;
        }
        return false;
    }

    pub(crate) fn move_left(&mut self) {
        execute!(self.stdout, cursor::MoveLeft(1)).unwrap();
    }
    pub(crate) fn move_right(&mut self) {
        // 获取光标位置
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let current_line_number = self.start_line + cursor_height as usize;
        let current_line_info = match self.file_content.content.get(&current_line_number) {
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
        execute!(self.stdout, cursor::MoveRight(1)).unwrap();
    }
    pub(crate) fn scroll_up(&mut self) {
        execute!(self.stdout, terminal::ScrollUp(1)).unwrap();
    }
    pub(crate) fn scroll_down(&mut self) {
        execute!(self.stdout, terminal::ScrollDown(1)).unwrap();
    }
}
