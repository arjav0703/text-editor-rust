use anyhow::Result;
use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Debug, Default, Clone)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<()> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code
                    && c == 'q'
                {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
