mod command_executor;
mod cursor_controller;
pub mod render;

use crate::args;
use crate::editor::render::Render;
use crate::files::reader::FileContent;
use clap::Parser;
use crossterm::cursor;
use crossterm::execute;
use std::io::stdout;

pub(crate) struct Editor {
    pub(crate) terminal_height: usize,
    pub(crate) terminal_width: usize,
    pub(crate) content_height: usize,
    pub(crate) start_line: usize,
    pub(crate) line_number_len: usize,
    pub(crate) file_content: FileContent,
    pub(crate) is_command_mode: bool,
    pub(crate) command_line: String,
}

impl Editor {
    pub(crate) fn new() -> Self {
        let args = args::Args::parse();
        let file_path = args.file_path.unwrap_or_else(|| "test1.txt".to_string());
        let file_content = FileContent::read(&file_path);
        let (terminal_width, terminal_height) = crossterm::terminal::size().unwrap();
        Self {
            terminal_height: (terminal_height - 1) as usize,
            terminal_width: terminal_width as usize,
            content_height: (terminal_height - 2) as usize,
            start_line: 0,
            line_number_len: 1,
            file_content,
            is_command_mode: false,
            command_line: String::new(),
        }
    }
}

impl Editor {
    pub(crate) fn show_content(&mut self) -> Result<(), String> {
        Render::render_all(self, 0);
        Ok(())
    }
}

impl Editor {
    pub(crate) fn enable_mouse_capture(&mut self) {
        // 启动鼠标捕获
        execute!(stdout(), crossterm::event::EnableMouseCapture).unwrap();
    }

    pub(crate) fn switch_command_mode(&mut self, mode: bool) {
        self.is_command_mode = mode;
    }

    pub(crate) fn append_command_line(&mut self, c: char) {
        self.command_line.push(c);
        // Render::render_command_line(&mut self.render.stdout, self);
    }

    pub(crate) fn pop_command_line(&mut self) {
        self.command_line.pop();
        // Render::render_command_line(&mut self.render.stdout, self);
        if self.command_line.len() == 0 {
            self.switch_command_mode(false);
        }
    }
}

impl Editor {
    pub(crate) fn reset_cursor() {
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
}
