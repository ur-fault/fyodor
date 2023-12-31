use std::{
    cell::RefCell,
    io::{self, stdout, Write},
    panic,
    rc::Rc, thread,
};

use crossterm::{event::Event, execute, style::ContentStyle, QueueableCommand};

use crate::{
    canvas::{Canvas, CanvasLike},
    cell::Cell,
    layout::Dims,
};

use super::helpers::term_size;

pub struct RenderSpace {
    shown: Canvas,
    hidden: Canvas,
}

impl RenderSpace {
    pub fn new(size: Dims) -> Self {
        Self {
            shown: Canvas::from_dims(size),
            hidden: Canvas::from_dims(size),
        }
    }

    pub fn canvas(&self) -> Canvas {
        self.hidden.clone()
    }

    pub fn other(&self) -> Canvas {
        self.shown.clone()
    }

    pub fn both_mut(&mut self) -> (&mut Canvas, &mut Canvas) {
        (&mut self.hidden, &mut self.shown)
    }

    fn on_resize(&mut self, size: impl Into<Dims>) -> io::Result<()> {
        let size = size.into();
        self.shown.resize(size);
        self.hidden.resize(size);

        Ok(())
    }
}

pub type SharedRenderSpace = Rc<RefCell<RenderSpace>>;

impl CanvasLike for SharedRenderSpace {
    fn set(&mut self, pos: Dims, cell: Cell) {
        self.borrow_mut().canvas().set(pos, cell)
    }

    fn pos(&self) -> Dims {
        self.borrow().canvas().pos()
    }

    fn size(&self) -> Dims {
        self.borrow().canvas().size()
    }
}

// TODO: allow only one renderer at a time by using a global variable

pub struct Renderer {
    size: Dims,
    render_space: SharedRenderSpace,
    full_redraw: bool,
}

impl Renderer {
    pub fn new() -> io::Result<Self> {
        let size = term_size();

        let mut ren = Renderer {
            size,
            render_space: Rc::new(RefCell::new(RenderSpace::new(size))),
            full_redraw: true,
        };

        ren.register_panic_hook();
        ren.turn_on()?;

        Ok(ren)
    }

    fn register_panic_hook(&self) {
        // TODO: cache the old hook and set it after disabling ours
        panic::set_hook(Box::new(move |panic_info| {
            let mut stdout = stdout();

            execute!(
                stdout,
                crossterm::terminal::LeaveAlternateScreen,
                crossterm::cursor::Show
            )
            .unwrap();

            crossterm::terminal::disable_raw_mode().unwrap();

            better_panic::Settings::auto().create_panic_handler()(panic_info);
        }));
    }

    fn unregiser_panic_hook(&self) {
        if !thread::panicking() {
            let _ = panic::take_hook();
        }
    }

    fn turn_on(&mut self) -> io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            stdout(),
            crossterm::cursor::Hide,
            crossterm::terminal::EnterAlternateScreen,
        )?;

        self.on_resize(None)?;

        Ok(())
    }

    pub fn turn_off(self) {} // we drop self, which calls internal version

    fn turn_off_internal(&mut self) -> io::Result<()> {
        crossterm::execute!(
            stdout(),
            crossterm::cursor::Show,
            crossterm::terminal::LeaveAlternateScreen,
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    fn on_resize(&mut self, size: Option<Dims>) -> io::Result<()> {
        self.size = size.unwrap_or_else(|| term_size());
        self.render_space.borrow_mut().on_resize(self.size)?;
        self.full_redraw = true;

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event) -> io::Result<()> {
        if let Event::Resize(x, y) = event {
            self.on_resize(Some((*x as i32, *y as i32).into()))?
        }

        // TODO: exit on CTRL+C

        Ok(())
    }

    // TODO: rename to something like `current_frame` since it's valid only for one frame
    pub fn canvas(&self) -> Canvas {
        self.render_space.borrow().canvas().clone()
    }

    pub fn get_render_space(&self) -> SharedRenderSpace {
        self.render_space.clone()
    }

    pub fn render(&mut self) -> io::Result<()> {
        let mut tty = stdout();

        let mut style = ContentStyle::default();
        tty.queue(crossterm::style::ResetColor)?;

        // TODO: save render space into variable

        for y in 0..self.size.y {
            if self.render_space.borrow().canvas().get_buf().buf_ref()[y as usize]
                == self.render_space.borrow().other().get_buf().buf_ref()[y as usize]
                && !self.full_redraw
            {
                continue;
            }

            tty.queue(crossterm::cursor::MoveTo(
                0,
                y.clamp(u16::MIN as i32, u16::MAX as i32) as u16,
            ))?;

            for x in 0..self.size.x {
                if let Cell::Content(c) =
                    &self.render_space.borrow().canvas().get_buf().buf_ref()[y as usize][x as usize]
                {
                    if style != c.style {
                        // TODO: rewrite using `if let Some(...) = ...
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

        {
            let mut binding = self.render_space.borrow_mut();
            let (hidden, shown) = binding.both_mut();
            std::mem::swap(hidden, shown);
        }

        self.render_space.borrow_mut().canvas().clear();

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.unregiser_panic_hook();
        let _ = self.turn_off_internal();
    }
}
