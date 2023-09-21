mod render;
mod cursor_controller;

use crossterm::{execute};
use crate::files::reader::FileContent;

pub(crate) struct Editor {
    pub(crate) terminal_height: usize,
    pub(crate) terminal_width: usize,
    pub(crate) content_height: usize,
    pub(crate) stdout: std::io::Stdout,
    pub(crate) start_line: usize,
    pub(crate) file_content: FileContent,
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
            file_content,
        }
    }
}

impl Editor {
    pub(crate) fn show_content(&mut self) -> Result<(), String> {
        render::render_all(self, 0);
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
        execute!(self.stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    }
}
