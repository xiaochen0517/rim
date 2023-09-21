use crate::editor::Editor;
use crossterm::{event};
use crossterm::event::{Event, KeyCode, KeyEventKind, MouseEventKind};

pub(crate) fn run(current_editor: &mut Editor) {
    current_editor.enable_mouse_capture();
    loop {
        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => {
                        current_editor.clean_screen();
                        break;
                    }
                    KeyCode::Up | KeyCode::Char('k') => current_editor.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => current_editor.move_down(),
                    KeyCode::Left | KeyCode::Char('h') => current_editor.move_left(),
                    KeyCode::Right | KeyCode::Char('l') => current_editor.move_right(),
                    KeyCode::Char('f') => current_editor.scroll_up(),
                    _ => {}
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