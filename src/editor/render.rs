use std::io::{Stdout, Write};
use std::iter;
use crossterm::execute;
use crossterm::cursor;
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crate::editor::Editor;
use crate::files::reader::Line;

pub(crate) fn render_all(editor: &mut Editor, start_line: usize) {
    // 清空屏幕
    editor.clean_screen();
    let stdout = &mut editor.stdout;
    let file_content = &editor.file_content;
    // 获取终端宽高
    let mut end_index = 0;
    let edit_height_size = editor.terminal_height - 1;
    for index in start_line..(file_content.content.len() - 1) {
        // 如果当前行数大于屏幕高度-1，则退出循环
        let render_terminal_line = index - start_line;
        if render_terminal_line > edit_height_size {
            end_index = index;
            break;
        }
        // 获取当前行信息并判断当前行是否存在
        match file_content.content.get(&index) {
            // 如果存在，则打印当前行
            Some(line_info) => {
                end_index = index;
                render_content_line(stdout, render_terminal_line as u16, line_info);
            }
            None => {
                break;
            }
        }
    }
    if end_index < edit_height_size {
        for index in end_index + 1..edit_height_size {
            render_empty_line(stdout, index as u16);
        }
    }
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    render_command_line(editor);
    editor.start_line = start_line;
}

pub(crate) fn render_scroll_down(editor: &mut Editor) {
    // 获取当前光标位置
    let cursor_position = cursor::position().unwrap().clone();
    let stdout = &mut editor.stdout;
    let file_content = &editor.file_content;
    let current_start_line = editor.start_line - 1;
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    let first_line = file_content.content.get(&current_start_line);
    match first_line {
        Some(line_info) => render_content_line(stdout, 0, line_info),
        None => {
            return;
        }
    }
    editor.start_line -= 1;
    execute!(stdout, cursor::MoveTo(cursor_position.0, cursor_position.1)).unwrap();
    render_command_line(editor);
}

pub(crate) fn render_scroll_up(editor: &mut Editor) {
    // 获取当前光标位置
    let cursor_position = cursor::position().unwrap().clone();
    let stdout = &mut editor.stdout;
    let file_content = &editor.file_content;
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout, cursor::MoveTo(0, cursor_position.1)).unwrap();
    match file_content.content.get(&(editor.start_line + editor.content_height + 1)) {
        Some(line_info) => {
            render_content_line(stdout, editor.content_height as u16, line_info);
        }
        None => {
            return;
        }
    }
    editor.start_line += 1;
    execute!(stdout, cursor::MoveTo(cursor_position.0, cursor_position.1)).unwrap();
    render_command_line(editor);
}

pub(crate) fn render_content_line(stdout: &mut Stdout, line_number: u16, line_info: &Line) {
    execute!(stdout, cursor::MoveTo(0, line_number)).unwrap();
    execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
    if line_info.is_wrapped {
        print!("-:");
    } else {
        print!("{}:", line_info.line_number);
    }
    execute!(stdout, SetForegroundColor(Color::Reset)).unwrap();
    print!("{}", line_info.text);
    stdout.flush().unwrap();
}

pub(crate) fn render_empty_line(stdout: &mut Stdout, line_number: u16) {
    execute!(stdout, cursor::MoveTo(0, line_number)).unwrap();
    execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
    print!("~");
    stdout.flush().unwrap();
}

pub(crate) fn render_command_line(editor: &mut Editor) {
    let stdout = &mut editor.stdout;
    // 获取当前光标位置
    let cursor_position = cursor::position().unwrap().clone();
    // 设置命令行背景色
    execute!(
            stdout,
            cursor::MoveTo(0, editor.terminal_height as u16),
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        ).unwrap();
    print!(":");
    print!("{}", iter::repeat(' ').take(editor.terminal_width as usize - 1).collect::<String>());
    stdout.flush().unwrap();
    execute!(
            stdout,
            cursor::MoveTo(cursor_position.0, cursor_position.1),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        ).unwrap();
}