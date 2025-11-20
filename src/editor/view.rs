use super::*;

pub struct View {}

impl View {
    pub fn render() -> Result<()> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("Hello, World!\r\n")?;

        for current_row in 1..height {
            Terminal::clear_line()?;

            if current_row == height / 3 {
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
