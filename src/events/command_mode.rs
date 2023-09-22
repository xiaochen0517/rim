use crate::editor::Editor;
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn handle_event(key: KeyEvent, editor: &mut Editor) {
    match key.code {
        KeyCode::Char(char) => {
            editor.append_command_line(char);
        }
        KeyCode::Backspace => {
            if editor.is_command_mode {
                editor.pop_command_line();
            }
        }
        KeyCode::Enter => {
            editor.execute_command();
        }
        _ => {}
    }
}
