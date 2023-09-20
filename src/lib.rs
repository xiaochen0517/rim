mod args;
mod editor;
mod files;

use std::io::{Cursor, Write};
use clap::Parser;
use crossterm::{cursor, event, QueueableCommand};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::files::reader;
use crate::editor::Editor;

pub fn run() {
    let args = args::Args::parse();
    let file_path = args.file_path.unwrap_or_else(|| "test.txt".to_string());
    let file_content = reader::FileContent::read(&file_path);
    let mut current_editor = Editor::new();
    current_editor.show_content(&file_content).unwrap();
    loop {
        if let Ok(key_event) = event::read() {
            if let Event::Key(key) = key_event {
                if key.kind == KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Char('k') => current_editor.move_up(1),
                    KeyCode::Down | KeyCode::Char('j') => current_editor.move_down(1),
                    KeyCode::Left | KeyCode::Char('h') => current_editor.move_left(1),
                    KeyCode::Right | KeyCode::Char('l') => current_editor.move_right(1),
                    _ => {}
                }
            }
        }
    }
}