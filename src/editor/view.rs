use super::*;
use crate::editor::command::{Direction, EditorCommand};
pub mod buffer;
use buffer::Buffer;
use unicode_segmentation::UnicodeSegmentation;

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
    pub size: Size,
    pub scroll_offset: Coords,
}

impl View {
    pub fn render(&self) -> Result<()> {
        let Size { height, width } = Terminal::size()?;
        let buf_len = self.buffer.content.len();
        let top = self.scroll_offset.y;

        for current_row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self
                .buffer
                .content
                .get(current_row.saturating_add(top) as usize)
            {
                let left = self.scroll_offset.x as usize;
                let right = (self.scroll_offset.x.saturating_add(width)) as usize;
                if let Some(slice) = line.get(left..right) {
                    Self::render_line(current_row.into(), slice)?;
                } else {
                    Self::render_line(current_row.into(), line)?;
                }
            } else if current_row == height / 3 && buf_len == 0 {
                Self::render_welcome_message()?;
            } else {
                Self::render_empty_row()?;
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn render_line(_at: usize, line_text: &str) -> Result<()> {
        let processed_text = Self::process_text(line_text);
        Terminal::print(&processed_text)?;
        Ok(())
    }

    fn process_text(text: &str) -> String {
        text.graphemes(true)
            .map(Self::replace_whitespace)
            .collect()
    }

    fn replace_whitespace(grapheme: &str) -> &str {
        match grapheme {
            " " => " ",
            "\t" => " ",
            g if !g.trim().is_empty() => g,
            g if g.chars().any(|c| c.is_control()) => "▯",
            g if !g.is_empty() => "␣",
            _ => "·",
        }
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
        let Coords { mut x, mut y } = self.position;

        match direction {
            Direction::Up => {
                if let Some(upper_line_len) = self
                    .buffer
                    .get_line_length(y.saturating_sub(1) as usize)
                    .into()
                {
                    let upper_line_length = upper_line_len as u16;
                    if x > upper_line_length {
                        x = upper_line_length;
                    }
                }
                y = y.saturating_sub(1);
            }
            Direction::Down => {
                let is_at_bottom = self
                    .buffer
                    .content
                    .get(y.saturating_add(1) as usize)
                    .is_none();
                if is_at_bottom {
                    return;
                }

                if let Some(lower_line_len) = self
                    .buffer
                    .get_line_length(y.saturating_add(1) as usize)
                    .into()
                {
                    let lower_line_len = lower_line_len as u16;
                    if x > lower_line_len {
                        x = lower_line_len;
                    }
                }
                y = y.saturating_add(1);
            }
            Direction::Left => {
                if let Some(line_len) = self.buffer.get_line_length(y as usize).into() {
                    if (x as usize) > line_len {
                        x = x.saturating_sub(1);
                    } else if x == 0 {
                        if y == 0 {
                            return;
                        }
                        y = y.saturating_sub(1);
                        if let Some(prev_line_len) = self.buffer.get_line_length(y as usize).into()
                        {
                            x = prev_line_len as u16;
                        } else {
                            x = 0;
                        }
                    } else {
                        x = x.saturating_sub(1);
                    }
                } else {
                    x = x.saturating_sub(1);
                }
            }
            Direction::Right => {
                if let Some(line_len) = self.buffer.get_line_length(y as usize).into() {
                    if (x as usize) < line_len {
                        x = x.saturating_add(1);
                    } else {
                        x = 0;
                        y = y.saturating_add(1);
                    }
                } else {
                    x = x.saturating_add(1);
                }
            }
        }

        self.position = Coords { x, y };
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {
        let Coords { x, y } = self.position;
        let Size { width, height } = Terminal::size().unwrap_or(Size {
            height: 24,
            width: 80,
        });

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
        }

        // Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
}
