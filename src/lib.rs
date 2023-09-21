mod args;
mod editor;
mod events;
mod files;

use crate::editor::Editor;
use crate::files::reader;
use clap::Parser;

pub fn run() {
    let args = args::Args::parse();
    let file_path = args.file_path.unwrap_or_else(|| "test.txt".to_string());
    let file_content = reader::FileContent::read(&file_path);
    let mut current_editor = Editor::new(file_content);
    current_editor.show_content().unwrap();
    events::run(&mut current_editor);
}
