use std::io::Write;
use crate::files::reader::FileContent;
use std::iter;
use crossterm::execute;
use crossterm::cursor;
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};

pub(crate) fn render(stdout: &mut std::io::Stdout, start_line: usize, file_content: &FileContent) {
    // 清空屏幕
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).expect("Failed to clear screen");
    let (width, height) = crossterm::terminal::size().unwrap();
    let mut end_index = 0;
    let mut last_line_number = 0;
    for (index, line_info) in file_content.content.iter().enumerate().skip(start_line) {
        if index - start_line >= height as usize {
            break;
        }
        end_index = index;
        execute!(stdout, cursor::MoveTo(0, (index - start_line) as u16)).unwrap();
        execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
        if last_line_number != line_info.line_number {
            last_line_number = line_info.line_number;
            print!("{}:", line_info.line_number);
        } else {
            print!("-:");
        }
        execute!(stdout, SetForegroundColor(Color::Reset)).unwrap();
        print!("{}", line_info.text);
        stdout.flush().unwrap();
    }
    let edit_height_size = (height - 1) as usize;
    if end_index < edit_height_size {
        for index in end_index + 1..edit_height_size {
            execute!(stdout, cursor::MoveTo(0, index as u16)).unwrap();
            execute!(stdout, SetForegroundColor(Color::Blue)).unwrap();
            print!("~");
            stdout.flush().unwrap();
        }
    }
    // 设置命令行背景色
    execute!(
            stdout,
            cursor::MoveTo(0, height),
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        ).unwrap();
    print!(":");
    print!("{}", iter::repeat(' ').take(width as usize - 1).collect::<String>());
    stdout.flush().unwrap();
    execute!(
            stdout,
            cursor::MoveTo(0, 0),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        ).unwrap();
}