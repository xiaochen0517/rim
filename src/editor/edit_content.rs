use std::io::stdout;

use crossterm::cursor;

use crate::editor::render::Render;
use crate::editor::Editor;
use crate::files::reader::Line;

impl Editor {
    pub(crate) fn add_content_char(&mut self, char: char) {
        let mut stdout = stdout();
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let current_content_index = self.start_line + cursor_height as usize;
        // let mut file_content = &mut self.file_content;
        let current_line_info = match self.file_content.get_mut(current_content_index) {
            Some(line_info) => line_info,
            None => {
                let last_line_info = self.file_content.last_mut().unwrap();
                last_line_info.text.insert(last_line_info.text.len(), '\r');
                last_line_info.text.insert(last_line_info.text.len(), '\n');
                let new_line_number = last_line_info.line_number + 1;
                self.file_content
                    .push(Line::new(String::new(), new_line_number, false));
                self.file_content.last_mut().unwrap()
            }
        };
        current_line_info
            .text
            .insert(cursor_width as usize - self.line_number_len - 1, char);
        let edited_line_info = current_line_info.clone();
        let mut refresh_index_vec = Vec::new();
        if current_line_info.text.len() > self.terminal_width - self.line_number_len {
            self.set_line_wrapped(current_content_index, &mut refresh_index_vec);
        }
        Render::render_content_line(
            &mut stdout,
            self.line_number_len,
            cursor_height,
            &edited_line_info,
        );
        for refresh_content_index in refresh_index_vec.into_iter() {
            let refresh_line_info = self.file_content.get(refresh_content_index).unwrap();
            Render::render_content_line(
                &mut stdout,
                self.line_number_len,
                cursor_height,
                refresh_line_info,
            );
        }
        self.move_right();
    }

    fn set_line_wrapped(&mut self, current_content_index: usize, refresh_vec: &mut Vec<usize>) {
        let (_cursor_width, cursor_height) = cursor::position().unwrap();
        let (new_line_string, line_number) = {
            let line_info = self.file_content.get_mut(current_content_index).unwrap();
            if line_info.text.len() <= self.terminal_width - self.line_number_len {
                return;
            }
            (
                line_info
                    .text
                    .split_off(self.terminal_width - self.line_number_len - 1),
                line_info.line_number,
            )
        };
        // 获取下一行内容
        let next_content_index = current_content_index + 1;
        match self.file_content.get_mut(next_content_index) {
            Some(line_info) => {
                if line_info.is_wrapped {
                    line_info.text.insert_str(0, &new_line_string);
                    refresh_vec.push(next_content_index - cursor_height as usize);
                    self.set_line_wrapped(next_content_index, refresh_vec);
                } else {
                    self.file_content.insert(
                        current_content_index,
                        Line::new(new_line_string, line_number, true),
                    );
                    refresh_vec.push(next_content_index - cursor_height as usize);
                }
            }
            None => {
                self.file_content
                    .push(Line::new(new_line_string, line_number, true));
                refresh_vec.push(next_content_index - cursor_height as usize);
            }
        };
    }

    pub(crate) fn delete_content_char(&mut self) {
        let mut stdout = stdout();
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let line_number_len = self.line_number_len;
        // 检查光标是否处于行首
        if cursor_width as usize == line_number_len + 1 {
            return;
        }
        // 获取当前行数
        let current_line_number = self.start_line + cursor_height as usize;
        // 获取当前行信息
        let current_line_info = match self.file_content.get_mut(current_line_number) {
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
}
