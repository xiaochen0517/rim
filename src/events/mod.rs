use crate::editor::Editor;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

use crate::editor::render::Render;

pub(crate) fn run(current_editor: &mut Editor) {
    current_editor.enable_mouse_capture();
    loop {
        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    continue;
                }
                if current_editor.is_command_mode {
                    match key.code {
                        KeyCode::Char(char) => {
                            current_editor.append_command_line(char);
                            continue;
                        }
                        KeyCode::Backspace => {
                            if current_editor.is_command_mode {
                                current_editor.pop_command_line();
                            }
                        }
                        KeyCode::Enter => {
                            current_editor.execute_command();
                        }
                        _ => {}
                    }
                }
                match key.code {
                    KeyCode::Char('q') => {
                        Editor::reset_cursor();
                        Render::clean_screen();
                        break;
                    }
                    KeyCode::Up | KeyCode::Char('k') => current_editor.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => current_editor.move_down(),
                    KeyCode::Left | KeyCode::Char('h') => current_editor.move_left(),
                    KeyCode::Right | KeyCode::Char('l') => current_editor.move_right(),
                    KeyCode::Char(':') => {
                        if key.modifiers != KeyModifiers::SHIFT {
                            continue;
                        }
                        if !current_editor.is_command_mode {
                            current_editor.switch_command_mode(true);
                        }
                        current_editor.append_command_line(':');
                    }
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
