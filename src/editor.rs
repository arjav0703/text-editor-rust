use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
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
        Self::clear_screen()?;
        enable_raw_mode()?;

        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if !self.running {
                break;
            }
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

    fn clear_screen() -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, 0))?;
        execute!(stdout, Clear(ClearType::All))?;
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        if !self.running {
            Self::clear_screen()?;
            println!("byeeeeee")
        }
        Ok(())
    }
}
