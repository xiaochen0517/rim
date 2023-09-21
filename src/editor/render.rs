use crate::editor::Editor;
use crate::files::reader::Line;
use crossterm::cursor;
use crossterm::execute;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;
use std::iter;

impl Editor {
    pub(crate) fn render_all(&mut self, start_line: usize) {
        // 清空屏幕
        self.clean_screen();
        let file_content = &(&(self.file_content)).content;
        // 获取终端宽高
        let mut end_index = 0;
        let edit_height_size = &self.terminal_height - 1;
        // 计算行号的最大长度
        self.line_number_len = match file_content.get(&(file_content.len() - 2)) {
            Some(line_info) => line_info.line_number.to_string().len(),
            None => 1,
        };
        // 渲染内容
        for index in start_line..file_content.len() {
            // 如果当前行数大于屏幕高度-1，则退出循环
            let render_terminal_line = index - start_line;
            if render_terminal_line > edit_height_size {
                end_index = index;
                break;
            }
            // 获取当前行信息并判断当前行是否存在
            match file_content.get(&index) {
                // 如果存在，则打印当前行
                Some(line_info) => {
                    end_index = index;
                    render_content_line(
                        &mut self.stdout,
                        self.line_number_len,
                        render_terminal_line as u16,
                        line_info,
                    );
                }
                None => {
                    break;
                }
            }
        }
        if end_index < edit_height_size {
            for index in end_index + 1..edit_height_size + 1 {
                render_empty_line(self, index as u16);
            }
        }
        execute!(self.stdout, cursor::MoveTo(0, 0)).unwrap();
        self.render_command_line();
        self.start_line = start_line;
    }

    pub(crate) fn render_scroll_down(&mut self) {
        // 获取当前光标位置
        let cursor_position = cursor::position().unwrap().clone();
        let current_start_line = self.start_line - 1;
        execute!(self.stdout, Clear(ClearType::CurrentLine)).unwrap();
        execute!(self.stdout, cursor::MoveTo(0, 0)).unwrap();
        let file_content = &self.file_content;
        let first_line = file_content.content.get(&current_start_line);
        match first_line {
            Some(line_info) => {
                render_content_line(&mut self.stdout, self.line_number_len, 0, line_info)
            }
            None => {
                return;
            }
        }
        self.start_line -= 1;
        execute!(
            self.stdout,
            cursor::MoveTo(cursor_position.0, cursor_position.1)
        )
        .unwrap();
        self.render_command_line();
    }

    pub(crate) fn render_scroll_up(&mut self) {
        // 获取当前光标位置
        let cursor_position = cursor::position().unwrap().clone();
        execute!(self.stdout, Clear(ClearType::CurrentLine)).unwrap();
        execute!(self.stdout, cursor::MoveTo(0, cursor_position.1)).unwrap();
        match self
            .file_content
            .content
            .get(&(self.start_line + self.content_height + 1))
        {
            Some(line_info) => {
                render_content_line(
                    &mut self.stdout,
                    self.line_number_len,
                    self.content_height as u16,
                    line_info,
                );
            }
            None => {
                return;
            }
        }
        self.start_line += 1;
        execute!(
            self.stdout,
            cursor::MoveTo(cursor_position.0, cursor_position.1)
        )
        .unwrap();
        self.render_command_line();
    }

    pub(crate) fn render_command_line(&mut self) {
        // 获取当前光标位置
        let cursor_position = cursor::position().unwrap().clone();
        // 设置命令行背景色
        execute!(
            self.stdout,
            cursor::MoveTo(0, (self.terminal_height + 1) as u16),
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        )
        .unwrap();
        print!("{}", self.command_line);
        print!(
            "{}",
            iter::repeat(' ')
                .take(self.terminal_width - self.command_line.len())
                .collect::<String>()
        );
        self.stdout.flush().unwrap();
        execute!(
            self.stdout,
            cursor::MoveTo(cursor_position.0, cursor_position.1),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        )
        .unwrap();
    }
}

pub(crate) fn render_content_line(
    stdout: &mut std::io::Stdout,
    line_number_len: usize,
    line_number: u16,
    line_info: &Line,
) {
    execute!(stdout, cursor::MoveTo(0, line_number)).unwrap();
    execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
    let mut line_number_str = line_info.line_number.to_string();
    if line_info.is_wrapped {
        line_number_str = "-".to_string();
    }
    if line_number_str.len() < line_number_len {
        line_number_str = iter::repeat(' ')
            .take(line_number_len - line_number_str.len())
            .collect::<String>()
            + &line_number_str;
    }
    print!("{}:", line_number_str);
    execute!(stdout, SetForegroundColor(Color::Reset)).unwrap();
    print!("{}", line_info.text);
    stdout.flush().unwrap();
}

pub(crate) fn render_empty_line(editor: &mut Editor, line_number: u16) {
    execute!(editor.stdout, cursor::MoveTo(0, line_number)).unwrap();
    execute!(editor.stdout, SetForegroundColor(Color::Blue)).unwrap();
    print!("~");
    editor.stdout.flush().unwrap();
}
