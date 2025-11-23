use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyEvent, KeyEventKind, read},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};
use std::io::{Write, stdout};
mod terminal;
use terminal::Terminal;
pub mod view;
use view::{Coords, View, buffer::Buffer};
mod command;
use command::EditorCommand;

#[derive(Debug, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}
impl Default for Size {
    fn default() -> Self {
        Terminal::size().unwrap_or_default()
    }
}

pub struct Editor {
    pub running: bool,
    pub view: View,
    pub file_path: String,
}

impl Default for Editor {
    fn default() -> Self {
        let test_buffer = Buffer {
            content: vec!["Hello, World!".to_string()],
        };

        Self {
            running: true,
            view: View {
                buffer: test_buffer,
                position: Coords::index(),
                scroll_offset: Coords::index(),
            },
            file_path: String::from("untitled.txt"),
        }
    }
}

impl Editor {
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
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = EditorCommand::try_from(event.clone()) {
                match command {
                    EditorCommand::Quit => {
                        self.running = false;
                    }
                    EditorCommand::Save => {
                        self.view.buffer.write_to_file(&self.file_path)?;
                    }
                    _ => {
                        self.view.handle_command(command);
                    }
                }
            }
        } else {
            panic!("Received unsupported event.");
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
            self.view.render()?;
            Terminal::move_cursor_to(Coords {
                x: self.view.position.x,
                y: self.view.position.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
