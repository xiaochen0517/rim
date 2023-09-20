use std::io::Write;
use std::iter;
use crossterm::execute;
use crossterm::cursor;
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crate::files::reader::FileContent;

pub(crate) struct Editor {
    stdout: std::io::Stdout,
}

impl Editor {
    pub(crate) fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
        }
    }
}

impl Editor {
    pub(crate) fn show_content(&mut self, content: &FileContent) -> Result<(), String> {
        // 清空屏幕
        execute!(self.stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).expect("Failed to clear screen");
        let (width, height) = crossterm::terminal::size().unwrap();
        let mut end_index = 0;
        let mut last_line_number = 0;
        for (index, line_info) in content.content.iter().enumerate() {
            if index >= height as usize {
                break;
            }
            end_index = index;
            execute!(self.stdout, cursor::MoveTo(0, index as u16)).unwrap();
            execute!(self.stdout, SetForegroundColor(Color::Blue)).unwrap();
            if last_line_number != line_info.line_number {
                last_line_number = line_info.line_number;
                print!("{}:", line_info.line_number);
            } else {
                print!("-:");
            }
            execute!(self.stdout, SetForegroundColor(Color::Reset)).unwrap();
            print!("{}", line_info.text);
            self.stdout.flush().unwrap();
        }
        let edit_height_size = (height - 1) as usize;
        if end_index < edit_height_size {
            for index in end_index + 1..edit_height_size {
                execute!(self.stdout, cursor::MoveTo(0, index as u16)).unwrap();
                execute!(self.stdout, SetForegroundColor(Color::Blue)).unwrap();
                print!("~");
                self.stdout.flush().unwrap();
            }
        }
        // 设置命令行背景色
        execute!(
            self.stdout,
            cursor::MoveTo(0, height),
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        ).unwrap();
        print!(":");
        print!("{}", iter::repeat(' ').take(width as usize - 1).collect::<String>());
        self.stdout.flush().unwrap();
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        ).unwrap();
        Ok(())
    }
}

impl Editor {
    pub(crate) fn move_up(&mut self, size: u16) {
        execute!(self.stdout, cursor::MoveUp(size)).unwrap();
    }
    pub(crate) fn move_down(&mut self, size: u16) {
        execute!(self.stdout, cursor::MoveDown(size)).unwrap();
    }
    pub(crate) fn move_left(&mut self, size: u16) {
        execute!(self.stdout, cursor::MoveLeft(size)).unwrap();
    }
    pub(crate) fn move_right(&mut self, size: u16) {
        execute!(self.stdout, cursor::MoveRight(size)).unwrap();
    }
}
