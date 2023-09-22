use crate::editor::Editor;
use crate::files::reader::Line;
use crossterm::cursor;
use crossterm::execute;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Stdout, Write};
use std::iter;

pub struct Render {}

impl Render {
    pub(crate) fn clean_screen() {
        // 清空屏幕
        execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
    }

    pub(crate) fn render_all(editor: &mut Editor, start_line: usize) {
        let mut stdout = stdout();
        // 清空屏幕
        Self::clean_screen();
        // 计算行号的最大长度
        Self::refresh_line_number_len(editor);
        // 渲染内容
        let file_content = &editor.file_content.content;
        for index in start_line..start_line + editor.terminal_height {
            // 如果当前行数大于屏幕高度-1，则退出循环
            let render_terminal_line = index - start_line;
            if render_terminal_line > editor.content_height {
                break;
            }
            // 获取当前行信息并判断当前行是否存在
            match file_content.get(&index) {
                // 如果存在，则打印当前行
                Some(line_info) => {
                    Self::render_content_line(
                        &mut stdout,
                        editor.line_number_len,
                        render_terminal_line as u16,
                        line_info,
                    );
                }
                None => Self::render_empty_line(&mut stdout, render_terminal_line as u16),
            }
        }
        Self::init_cursor(&mut stdout, editor);
        Self::render_command_line(&mut stdout, editor);
        editor.start_line = start_line;
    }

    fn refresh_line_number_len(editor: &mut Editor) {
        let file_content = &editor.file_content.content;
        editor.line_number_len = match file_content.get(&(file_content.len() - 2)) {
            Some(line_info) => line_info.line_number.to_string().len(),
            None => 1,
        };
    }

    fn init_cursor(stdout: &mut Stdout, editor: &mut Editor) {
        execute!(stdout, cursor::MoveTo(editor.line_number_len as u16 + 1, 0)).unwrap();
    }

    pub(crate) fn render_scroll_down(stdout: &mut Stdout, editor: &mut Editor) {
        // 获取当前光标位置
        let cursor_position = cursor::position().unwrap().clone();
        let current_start_line = editor.start_line - 1;
        execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        let file_content = &editor.file_content;
        let first_line = file_content.content.get(&current_start_line);
        match first_line {
            Some(line_info) => {
                Render::render_content_line(stdout, editor.line_number_len, 0, line_info)
            }
            None => {
                return;
            }
        }
        editor.start_line -= 1;
        execute!(stdout, cursor::MoveTo(cursor_position.0, cursor_position.1)).unwrap();
        Render::render_command_line(stdout, editor);
        Render::check_line_end(stdout, editor);
    }

    pub(crate) fn render_scroll_up(stdout: &mut Stdout, editor: &mut Editor) {
        // 获取当前光标位置
        let cursor_position = cursor::position().unwrap().clone();
        execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
        execute!(stdout, cursor::MoveTo(0, cursor_position.1)).unwrap();
        match editor
            .file_content
            .content
            .get(&(editor.start_line + editor.content_height + 1))
        {
            Some(line_info) => {
                Render::render_content_line(
                    stdout,
                    editor.line_number_len,
                    editor.content_height as u16,
                    line_info,
                );
            }
            None => {
                return;
            }
        }
        editor.start_line += 1;
        execute!(stdout, cursor::MoveTo(cursor_position.0, cursor_position.1)).unwrap();
        Render::render_command_line(stdout, editor);
        Render::check_line_end(stdout, editor);
    }

    pub(crate) fn render_command_line(stdout: &mut Stdout, editor: &Editor) {
        // 获取当前光标位置
        let cursor_position = cursor::position().unwrap().clone();
        // 设置命令行背景色
        execute!(
            stdout,
            cursor::MoveTo(0, (editor.terminal_height + 1) as u16),
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        )
        .unwrap();
        print!("{}", editor.command_line);
        print!(
            "{}",
            iter::repeat(' ')
                .take(editor.terminal_width - editor.command_line.len())
                .collect::<String>()
        );
        stdout.flush().unwrap();
        execute!(
            stdout,
            cursor::MoveTo(cursor_position.0, cursor_position.1),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        )
        .unwrap();
    }

    pub(crate) fn render_content_line(
        stdout: &mut Stdout,
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

    pub(crate) fn render_empty_line(stdout: &mut Stdout, line_number: u16) {
        execute!(stdout, cursor::MoveTo(0, line_number)).unwrap();
        execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
        print!(" ~");
        stdout.flush().unwrap();
    }

    pub(crate) fn check_line_end(stdout: &mut Stdout, editor: &Editor) -> bool {
        // 获取光标位置
        let (cursor_width, cursor_height) = cursor::position().unwrap();
        let cursor_width = cursor_width as usize;
        let current_line_number = editor.start_line + cursor_height as usize;
        let current_line_info = match editor.file_content.content.get(&current_line_number) {
            Some(line_info) => line_info,
            None => {
                execute!(
                    stdout,
                    cursor::MoveTo(editor.line_number_len as u16 + 1, cursor_height)
                )
                .unwrap();
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
        if cursor_width >= max_column_number {
            execute!(
                stdout,
                cursor::MoveTo(max_column_number as u16, cursor_height)
            )
            .unwrap();
            return true;
        }
        if cursor_width <= editor.line_number_len {
            execute!(
                stdout,
                cursor::MoveTo(editor.line_number_len as u16 + 1, cursor_height)
            )
            .unwrap();
            return true;
        }
        return false;
    }
}
