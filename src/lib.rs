mod args;
mod editor;
mod events;
mod files;

use crate::editor::Editor;

pub fn run() {
    let mut current_editor = Editor::new();
    current_editor.show_content().unwrap();
    events::run(&mut current_editor);
}
