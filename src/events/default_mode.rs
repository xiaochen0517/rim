use crate::editor::render::Render;
use crate::editor::Editor;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) fn handle_event(key: KeyEvent, current_editor: &mut Editor) -> Result<(), String> {
    match key.code {
        KeyCode::Char('q') => {
            Editor::reset_cursor();
            Render::clean_screen();
            return Err("".to_string());
        }
        KeyCode::Up | KeyCode::Char('k') => current_editor.move_up(),
        KeyCode::Down | KeyCode::Char('j') => current_editor.move_down(),
        KeyCode::Left | KeyCode::Char('h') => current_editor.move_left(),
        KeyCode::Right | KeyCode::Char('l') => current_editor.move_right(),
        KeyCode::Char(':') => {
            if key.modifiers != KeyModifiers::SHIFT {
                return Ok(());
            }
            if !current_editor.is_command_mode {
                current_editor.switch_command_mode(true);
            }
            current_editor.append_command_line(':');
        }
        KeyCode::Char('i') => {
            if key.modifiers != KeyModifiers::NONE {
                return Ok(());
            }
            current_editor.switch_edit_mode(true);
        }
        _ => {}
    }
    Ok(())
}
