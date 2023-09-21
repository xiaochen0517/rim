mod command_executor;
mod cursor_controller;
mod render;

use crate::files::reader::FileContent;
use crossterm::execute;
use std::rc::Rc;

pub(crate) struct Editor {
    pub(crate) terminal_height: usize,
    pub(crate) terminal_width: usize,
    pub(crate) content_height: usize,
    pub(crate) stdout: std::io::Stdout,
    pub(crate) start_line: usize,
    pub(crate) line_number_len: usize,
    pub(crate) file_content: Rc<FileContent>,
    pub(crate) is_command_mode: bool,
    pub(crate) command_line: String,
}

impl Editor {
    pub(crate) fn new(file_content: FileContent) -> Self {
        let (terminal_width, terminal_height) = crossterm::terminal::size().unwrap();
        Self {
            terminal_height: (terminal_height - 1) as usize,
            terminal_width: terminal_width as usize,
            content_height: (terminal_height - 2) as usize,
            stdout: std::io::stdout(),
            start_line: 0,
            line_number_len: 1,
            file_content: Rc::new(file_content),
            is_command_mode: false,
            command_line: String::new(),
        }
    }
}

impl Editor {
    pub(crate) fn show_content(&mut self) -> Result<(), String> {
        self.render_all(0);
        Ok(())
    }
}

impl Editor {
    pub(crate) fn enable_mouse_capture(&mut self) {
        // 启动鼠标捕获
        execute!(self.stdout, crossterm::event::EnableMouseCapture).unwrap();
    }

    pub(crate) fn clean_screen(&mut self) {
        // 清空屏幕
        execute!(
            self.stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
    }

    pub(crate) fn switch_command_mode(&mut self, mode: bool) {
        self.is_command_mode = mode;
    }

    pub(crate) fn append_command_line(&mut self, c: char) {
        self.command_line.push(c);
        self.render_command_line();
    }

    pub(crate) fn pop_command_line(&mut self) {
        self.command_line.pop();
        self.render_command_line();
        if self.command_line.len() == 0 {
            self.switch_command_mode(false);
        }
    }
}
