use crate::editor::Editor;
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn handle_event(key: KeyEvent, editor: &mut Editor) {
    match key.code {
        KeyCode::Up => editor.move_up(),
        KeyCode::Down => editor.move_down(),
        KeyCode::Left => editor.move_left(),
        KeyCode::Right => editor.move_right(),
        KeyCode::Char(char) => {
            editor.add_content_char(char);
        }
        KeyCode::Backspace => {
            editor.delete_content_char();
        }
        KeyCode::Enter => {
            editor.line_feed();
        }
        KeyCode::Esc => {
            editor.switch_edit_mode(false);
        }
        _ => {}
    }
}
