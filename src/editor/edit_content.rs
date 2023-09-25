use crate::editor::render::Render;
use crate::editor::Editor;
use crate::files::reader::Line;
use crossterm::cursor;
use std::io::stdout;

impl Editor {
    pub(crate) fn add_content_char(&mut self, char: char) {
        let mut stdout = stdout();
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let line_number_len = self.line_number_len;
        // 获取当前行信息
        let current_line_info = match Self::get_current_line_info(self) {
            Some(line_info) => line_info,
            None => {
                // 获取最后一行信息
                let _last_line_info = self.file_content.content.last().unwrap();
                return;
            }
        };
        // 添加字符
        current_line_info
            .text
            .insert(cursor_width as usize - line_number_len - 1, char);
        // 更新当前行信息
        Render::render_content_line(
            &mut stdout,
            line_number_len,
            cursor_height,
            &current_line_info,
        );
        // 更新光标位置
        self.move_right();
    }

    pub(crate) fn delete_content_char(&mut self) {
        let mut stdout = stdout();
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let line_number_len = self.line_number_len;
        // 检查光标是否处于行首
        if cursor_width as usize == line_number_len + 1 {
            return;
        }
        // 获取当前行信息
        let current_line_info = match Self::get_current_line_info(self) {
            Some(line_info) => line_info,
            None => {
                return;
            }
        };
        // 删除字符
        current_line_info
            .text
            .remove(cursor_width as usize - line_number_len - 2);
        // 更新当前行信息
        Render::render_content_line(
            &mut stdout,
            line_number_len,
            cursor_height,
            &current_line_info,
        );
        // 更新光标位置
        self.move_left();
    }

    pub(crate) fn line_feed(&mut self) {}

    pub(crate) fn get_current_line_info(editor: &mut Editor) -> Option<&mut Line> {
        let (_cursor_width, cursor_height) = cursor::position().unwrap();
        // 获取当前行数
        let current_line_number = editor.start_line + cursor_height as usize;
        // 获取当前行信息
        editor.file_content.content.get_mut(current_line_number)
    }
}
