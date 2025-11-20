use anyhow::Result;
use core::cmp::min;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        Event::{self, Key},
        KeyCode::{self, Char},
        KeyEvent, KeyModifiers, read,
    },
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use std::io::{Write, stdout};
mod terminal;
use terminal::Terminal;

pub struct Size {
    pub height: u16,
    pub width: u16,
}

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

#[derive(Debug, Clone)]
pub struct Editor {
    running: bool,
    position: Coords,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            running: true,
            position: Coords::index(),
        }
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
            self.evaluate_event(&event)?;
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<()> {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.running = false;
                }
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    self.move_point(*code)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Coords::index())?;

        if !self.running {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\n")?;
        } else {
            self.draw_tildes()?;
            Terminal::move_cursor_to(Coords {
                x: self.position.x,
                y: self.position.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_tildes(&self) -> Result<()> {
        let Size { height, width: _w } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn move_point(&mut self, keycode: KeyCode) -> Result<()> {
        let Coords { mut x, mut y } = self.position;
        let Size { height, width } = Terminal::size()?;

        match keycode {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            _ => {}
        }

        self.position = Coords { x, y };
        Ok(())
    }
}
