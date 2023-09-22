mod command_mode;
mod default_mode;
mod editor_mode;

use crate::editor::Editor;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind, MouseEventKind};

pub(crate) fn run(current_editor: &mut Editor) {
    current_editor.enable_mouse_capture();
    loop {
        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    continue;
                }
                if current_editor.is_editor_mode {
                    editor_mode::handle_event(key, current_editor);
                    continue;
                }
                if current_editor.is_command_mode {
                    command_mode::handle_event(key, current_editor);
                    continue;
                }
                match default_mode::handle_event(key, current_editor) {
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
            if let Event::Mouse(mouse) = event {
                match mouse.kind {
                    MouseEventKind::ScrollUp => current_editor.move_up(),
                    MouseEventKind::ScrollDown => current_editor.move_down(),
                    _ => {}
                }
            }
        }
    }
}
