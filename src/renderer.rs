use std::io::{stdout, Write};

use crossterm::{event::Event, style::ContentStyle, QueueableCommand, Result as CRResult};
use unicode_width::UnicodeWidthChar;

use crate::canvas::Canvas;

use super::helpers::term_size;

pub type Dims = (i32, i32);

pub struct Renderer {
    size: Dims,
    shown: Canvas,
    hidden: Canvas,
    full_redraw: bool,
}

impl Renderer {
    pub fn new() -> CRResult<Self> {
        let size = term_size();
        let size = (size.0 as i32, size.1 as i32);

        let hidden = Canvas::from_dims(size);
        let shown = Canvas::from_dims(size);

        let mut ren = Renderer {
            size,
            shown,
            hidden,
            full_redraw: true,
        };

        ren.turn_on()?;

        Ok(ren)
    }

    fn turn_on(&mut self) -> CRResult<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            stdout(),
            crossterm::cursor::Hide,
            crossterm::terminal::EnterAlternateScreen,
        )?;

        self.on_resize(None)?;

        Ok(())
    }

    fn turn_off(&mut self) -> CRResult<()> {
        crossterm::execute!(
            stdout(),
            crossterm::cursor::Show,
            crossterm::terminal::LeaveAlternateScreen,
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    fn on_resize(&mut self, size: Option<Dims>) -> CRResult<()> {
        self.size = size.unwrap_or_else(|| term_size());
        self.shown.resize(self.size);
        self.hidden.resize(self.size);
        self.full_redraw = true;

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event) -> CRResult<()> {
        if let Event::Resize(x, ref y) = event {
            self.on_resize(Some((*x as i32, *y as i32)))?
        }

        Ok(())
    }

    pub fn canvas(&mut self) -> Canvas {
        self.hidden.clone()
    }

    pub fn render(&mut self) -> CRResult<()> {
        let mut tty = stdout();

        let mut style = ContentStyle::default();
        tty.queue(crossterm::style::ResetColor)?;

        for y in 0..self.size.1 {
            if self.hidden.get_buf().buf_ref()[y as usize]
                == self.shown.get_buf().buf_ref()[y as usize]
                && !self.full_redraw
            {
                continue;
            }

            tty.queue(crossterm::cursor::MoveTo(
                0,
                y.max(0).min(u16::MAX as i32) as u16, // clamp i32 to u16 range
            ))?;

            for x in 0..self.size.0 {
                if let Cell::Content(c) = &self.hidden.get_buf().buf_ref()[y as usize][x as usize] {
                    if style != c.style {
                        if style.background_color != c.style.background_color {
                            match c.style.background_color {
                                Some(x) => {
                                    tty.queue(crossterm::style::SetBackgroundColor(x))?;
                                }
                                None => {
                                    tty.queue(crossterm::style::SetBackgroundColor(
                                        crossterm::style::Color::Reset,
                                    ))?;
                                }
                            }
                        }
                        if style.foreground_color != c.style.foreground_color {
                            match c.style.foreground_color {
                                Some(x) => {
                                    tty.queue(crossterm::style::SetForegroundColor(x))?;
                                }
                                None => {
                                    tty.queue(crossterm::style::SetForegroundColor(
                                        crossterm::style::Color::Reset,
                                    ))?;
                                }
                            }
                        }
                        if style.attributes != c.style.attributes {
                            tty.queue(crossterm::style::SetAttribute(
                                crossterm::style::Attribute::Reset,
                            ))?;
                            if let Some(x) = c.style.foreground_color {
                                tty.queue(crossterm::style::SetForegroundColor(x))?;
                            }
                            if let Some(x) = c.style.background_color {
                                tty.queue(crossterm::style::SetBackgroundColor(x))?;
                            }
                            tty.queue(crossterm::style::SetAttributes(c.style.attributes))?;
                        }
                        style = c.style;
                    }
                    tty.queue(crossterm::style::Print(c.character))?;
                }
            }
        }

        tty.flush()?;
        self.full_redraw = false;

        std::mem::swap(&mut self.shown, &mut self.hidden);

        self.hidden.clear();

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.turn_off();
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct CellContent {
    pub character: char,
    pub width: u8,
    pub style: ContentStyle,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum Cell {
    #[default]
    PlaceHolder,
    Content(CellContent),
}

impl Cell {
    pub fn styled(c: char, s: ContentStyle) -> Self {
        Cell::Content(CellContent {
            character: c,
            width: c.width().unwrap_or(1) as u8,
            style: s,
        })
    }

    pub fn new(c: char) -> Self {
        Cell::styled(c, ContentStyle::default())
    }
}