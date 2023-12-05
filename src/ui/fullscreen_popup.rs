use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    canvas::{CanvasLike, CanvasLikeExt},
    drawable::Drawable,
    layout::{
        align::Align,
        sized::{KnownHeight, KnownWidth},
        Pos,
    },
    renderer::Renderer,
};

use super::{popup::Popup, Window};

pub struct FullScreenPopup(pub Popup);

impl FullScreenPopup {
    pub fn new(popup: Popup) -> Self {
        Self(popup)
    }
}

impl Window for FullScreenPopup {
    type Output = io::Result<KeyCode>;

    fn run(&mut self, renderer: &mut Renderer) -> Self::Output {
        loop {
            renderer
                .get_render_space()
                .show((Align::Center, Align::Center), &mut *self);
            renderer.render()?;

            let event = read()?;
            if let Event::Key(KeyEvent { code, kind, .. }) = event {
                if kind != KeyEventKind::Release {
                    break Ok(code);
                }
            }

            renderer.on_event(&event)?;
        }
    }
}

impl Drawable for &mut FullScreenPopup {
    type X = Align;
    type Y = Align;

    fn draw(&self, pos: impl Into<Pos<Align, Align>>, canvas: &mut impl CanvasLike) {
        <&Popup as Drawable>::draw(&&self.0, pos, canvas);
    }
}

impl KnownWidth for FullScreenPopup {
    fn w(&self) -> i32 {
        self.0.w()
    }
}

impl KnownHeight for FullScreenPopup {
    fn h(&self) -> i32 {
        self.0.h()
    }
}
