use super::*;

pub struct Terminal {}
impl Terminal {
    pub fn clear_line() -> Result<()> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn clear_screen() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Coords) -> Result<()> {
        let Coords { x, y } = position;
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<Size> {
        let (width, height) = size()?;
        let size = Size { height, width };
        Ok(size)
    }

    pub fn hide_cursor() -> Result<()> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<()> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<()> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn execute() -> Result<()> {
        stdout().flush()?;
        Ok(())
    }
}
