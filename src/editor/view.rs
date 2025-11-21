use super::*;
use crate::editor::command::{Direction, EditorCommand};
pub mod buffer;

#[derive(Debug, Clone)]
pub struct Coords {
    pub x: u16,
    pub y: u16,
}

impl Coords {
    pub fn index() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Coords {
    fn default() -> Self {
        Coords::index()
    }
}

#[derive(Debug, Clone, Default)]
pub struct View {
    pub buffer: Buffer,
    pub position: Coords,
}

impl View {
    pub fn render(&self) -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        let buf_len = self.buffer.content.len();

        for current_row in 0..height {
            Terminal::clear_line()?;

            if (current_row as usize) < buf_len {
                let line = &self.buffer.content[current_row as usize];
                Terminal::print(line)?;
            } else {
                if current_row == height / 3 && buf_len == 0 {
                    Self::render_welcome_message()?;
                } else {
                    Self::render_empty_row()?;
                }
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn render_welcome_message() -> Result<()> {
        let mut welcome_message = "Text editor -- version 1".to_string();
        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        let padding = (width.saturating_sub(len.try_into().expect("err"))) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1).into());

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width.into());

        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn render_empty_row() -> Result<()> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_cursor(direction),
            EditorCommand::Resize(_new_size) => {
                // Handle resize if needed
            }
            EditorCommand::Quit => {
                // Handled in Editor
            }
        }
    }

    fn move_cursor(&mut self, direction: Direction) {
        let Size { height, width } = Terminal::size().unwrap_or(Size {
            height: 24,
            width: 80,
        });
        match direction {
            Direction::Up => {
                self.position.y = self.position.y.saturating_sub(1);
            }
            Direction::Down => {
                self.position.y = self
                    .position
                    .y
                    .saturating_add(1)
                    .min(height.saturating_sub(1));
            }
            Direction::Left => {
                self.position.x = self.position.x.saturating_sub(1);
            }
            Direction::Right => {
                self.position.x = self
                    .position
                    .x
                    .saturating_add(1)
                    .min(width.saturating_sub(1));
            }
        }
    }
}
