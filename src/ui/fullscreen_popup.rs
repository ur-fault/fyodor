use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    canvas::{CanvasLike, CanvasLikeExt},
    drawable::Drawable,
    layout::{
        sized::{Aligned, Anchor, KnownHeight, KnownWidth},
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
            renderer.get_render_space().draw(
                Pos::new(
                    Aligned::new_x(Anchor::Center, self),
                    Aligned::new_y(Anchor::Center, self),
                ),
                &mut *self,
            );
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
    type X = Aligned;
    type Y = Aligned;

    fn draw(self, pos: impl Into<Pos<Aligned, Aligned>>, canvas: &mut impl CanvasLike) {
        self.0.draw(pos, canvas);
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
