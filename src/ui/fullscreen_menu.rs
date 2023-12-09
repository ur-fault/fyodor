use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use thiserror::Error;

use crate::{
    input::Keylist,
    layout::{
        align::Align,
        sized::{FullyKnown, KnownHeight, KnownWidth},
        Pos,
    },
    CanvasLike, CanvasLikeExt, Drawable, Renderer,
};

use super::{menu::Menu, Window};

#[derive(Debug, Error)]
pub enum MenuError {
    #[error("menu is empty")]
    Empty,
}

#[derive(Debug)]
pub struct MenuResult<'a, T> {
    pub code: KeyCode,
    pub index: usize,
    pub data: &'a T,
}

pub struct FullscreenMenu<T> {
    pub menu: Menu<T>,
    pub select_keys: Keylist,
    pub up_keys: Keylist,
    pub down_keys: Keylist,
}

impl<T> FullscreenMenu<T> {
    pub fn new(menu: Menu<T>) -> Self {
        Self {
            menu,
            select_keys: Keylist::new(false)
                .except_chars(&['s', 'S', 'k'])
                .except_chars(&['w', 'W', 'j']),
            up_keys: Keylist::new(false)
                .with_chars(&['s', 'S', 'k'])
                .with_keys(&[KeyCode::Up]),
            down_keys: Keylist::new(false)
                .with_chars(&['w', 'W', 'j'])
                .with_keys(&[KeyCode::Down]),
        }
    }
}

fn dok<T, E1, E2>(v: T) -> Result<Result<T, E1>, E2> {
    Ok(Ok(v))
}

impl<T> Window for FullscreenMenu<T>
where
    Self: Drawable<X = Align, Y = Align>,
{
    type Output<'a> = Result<MenuResult<'a, T>, MenuError> where T: 'a;

    fn run(&mut self, renderer: &mut Renderer) -> io::Result<Self::Output<'_>> {
        if self.menu.items().is_empty() {
            return Ok(Err(MenuError::Empty));
        }

        let mut canvas = renderer.get_render_space();
        loop {
            canvas.show((Align::Center, Align::Center), self);
            renderer.render()?;

            let event = crossterm::event::read()?;
            if let Event::Key(KeyEvent { code, kind, .. }) = event {
                if kind != KeyEventKind::Release {
                    if self.up_keys.contains(code) {
                        self.menu.up(1);
                    } else if self.down_keys.contains(code) {
                        self.menu.down(1);
                    } else if self.select_keys.contains(code) {
                        break dok(MenuResult {
                            code,
                            index: self.menu.selected_index().unwrap(),
                            data: self.menu.selected().unwrap(),
                        });
                    }
                }
            }

            renderer.on_event(&event)?;
        }
    }
}

impl<T> Drawable for FullscreenMenu<T>
where
    Menu<T>: Drawable<X = i32, Y = i32> + FullyKnown,
{
    type X = Align;
    type Y = Align;

    fn draw(&self, pos: impl Into<Pos<Align, Align>>, canvas: &mut impl CanvasLike) {
        let pos: Pos<_, _> = pos.into();
        let pos = pos.calc_both(self.menu.dims(), canvas.size());
        self.menu.draw(pos, canvas);
    }
}

impl<T> KnownWidth for FullscreenMenu<T>
where
    Menu<T>: KnownWidth,
{
    fn w(&self) -> i32 {
        self.menu.w()
    }
}

impl<T> KnownHeight for FullscreenMenu<T>
where
    Menu<T>: KnownHeight,
{
    fn h(&self) -> i32 {
        self.menu.h()
    }
}
