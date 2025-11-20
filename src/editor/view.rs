use super::*;
pub mod buffer;

#[derive(Debug, Clone, Default)]
pub struct View {
    pub buffer: Buffer,
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
}
