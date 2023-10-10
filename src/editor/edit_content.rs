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
        let mut refresh_line_number = 0;
        debug!("current_line_info: {:?}", current_line_info);
        debug!(
            "current_line_info.text.len(): {}",
            current_line_info.text.len()
        );
        debug!(
            "self.terminal_content_width: {}",
            self.terminal_width - self.line_number_len
        );
        if current_line_info.text.len() > self.terminal_width - self.line_number_len {
            refresh_line_number = self.set_line_wrapped(current_content_index, 0);
        }
        Render::render_content_line(
            &mut stdout,
            self.line_number_len,
            cursor_height,
            &edited_line_info,
        );
        debug!("refresh_line_number: {}", refresh_line_number);
        for refresh_add_index in 0..refresh_line_number {
            let refresh_terminal_index = cursor_height + refresh_add_index as u16 + 1;
            let refresh_content_index = current_content_index + refresh_add_index + 1;
            let refresh_line_info = self.file_content.get(refresh_content_index).unwrap();
            debug!(
                "refresh_terminal_index: {}, refresh_content_index: {}",
                refresh_terminal_index, refresh_content_index
            );
            debug!("refresh_line_info: {:?}", refresh_line_info);
            Render::render_content_line(
                &mut stdout,
                self.line_number_len,
                refresh_terminal_index,
                refresh_line_info,
            );
        }
        self.move_right();
    }

    fn set_line_wrapped(&mut self, current_content_index: usize, mut refresh_size: usize) -> usize {
        let (new_line_string, line_number) = {
            let line_info = self.file_content.get_mut(current_content_index).unwrap();
            if line_info.text.len() <= self.terminal_width - self.line_number_len {
                debug!("内容长度小于终端宽度，不需要换行");
                return refresh_size;
            }
            debug!("内容长度大于终端宽度，需要换行");
            (
                line_info
                    .text
                    .split_off(self.terminal_width - self.line_number_len - 1),
                line_info.line_number,
            )
        };
        // 获取下一行内容
        let next_content_index = current_content_index + 1;
        return match self.file_content.get_mut(next_content_index) {
            Some(line_info) => {
                if line_info.is_wrapped {
                    debug!("下一行已经换行，需要将内容插入到下一行的开头");
                    line_info.text.insert_str(0, &new_line_string);
                    refresh_size += 1;
                    self.set_line_wrapped(next_content_index, refresh_size)
                } else {
                    debug!("下一行未换行，需要将内容插入到一个新的行中");
                    self.file_content.insert(
                        current_content_index,
                        Line::new(new_line_string, line_number, true),
                    );
                    refresh_size += 1;
                    refresh_size
                }
            }
            None => {
                debug!("下一行不存在，需要将内容插入到一个新的行中");
                self.file_content
                    .push(Line::new(new_line_string, line_number, true));
                refresh_size += 1;
                refresh_size
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
