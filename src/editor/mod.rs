use std::io::stdout;

use clap::Parser;
use crossterm::execute;

use crate::args;
use crate::editor::render::Render;
use crate::files::reader;
use crate::files::reader::Line;

mod command_executor;
mod cursor_controller;
mod edit_content;
mod mode_switch;
pub mod render;

pub(crate) struct Editor {
    pub(crate) terminal_height: usize,
    pub(crate) terminal_width: usize,
    pub(crate) content_height: usize,
    pub(crate) start_line: usize,
    pub(crate) line_number_len: usize,
    pub(crate) file_content: Vec<Line>,
    pub(crate) is_command_mode: bool,
    pub(crate) command_line: String,
    pub(crate) is_editor_mode: bool,
}

impl Editor {
    pub(crate) fn new() -> Self {
        let args = args::Args::parse();
        let file_path = args.file_path.unwrap_or_else(|| "test1.txt".to_string());
        let file_content = reader::read(&file_path);
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
            is_editor_mode: false,
        }
    }

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
}
