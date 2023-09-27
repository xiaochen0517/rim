use crate::editor::render::Render;
use crate::editor::Editor;
use std::io::stdout;

impl Editor {
    pub(crate) fn switch_command_mode(&mut self, mode: bool) {
        self.is_command_mode = mode;
    }

    pub(crate) fn append_command_line(&mut self, c: char) {
        self.command_line.push(c);
        Render::render_command_line(&mut stdout(), self);
    }

    pub(crate) fn pop_command_line(&mut self) {
        self.command_line.pop();
        // Render::render_command_line(&mut self.render.stdout, self);
        if self.command_line.len() == 0 {
            self.switch_command_mode(false);
        }
    }

    pub(crate) fn switch_edit_mode(&mut self, mode: bool) {
        self.is_editor_mode = mode;
        if mode {
            self.command_line = "--INSERT--".to_string();
        } else {
            self.command_line = String::new();
        }
        Render::render_command_line(&mut stdout(), self);
    }
}
