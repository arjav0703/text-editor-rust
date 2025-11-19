use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use std::io::stdout;

#[derive(Debug, Default, Clone)]
pub struct Editor {
    running: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn run(&mut self) -> Result<()> {
        Terminal::clear_screen()?;
        enable_raw_mode()?;

        loop {
            self.refresh_screen()?;

            if !self.running {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.running = false;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<()> {
        if !self.running {
            Terminal::clear_screen()?;
            println!("byeeeeee")
        } else {
            self.draw_tildes()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn draw_tildes(&self) -> Result<()> {
        let (height, _width) = Terminal::size()?;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}

struct Terminal {}
impl Terminal {
    pub fn clear_screen() -> Result<()> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(x: u16, y: u16) -> Result<()> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16)> {
        Ok(size()?)
    }
}
