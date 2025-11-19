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
            Terminal::move_cursor_to(Coords::index())?;
        }
        Ok(())
    }

    fn draw_tildes(&self) -> Result<()> {
        let Size { height, width: _w } = Terminal::size()?;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Coords {
    pub x: u16,
    pub y: u16,
}

impl Coords {
    pub fn index() -> Self {
        Self { x: 0, y: 0 }
    }
}

struct Terminal {}
impl Terminal {
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
}
