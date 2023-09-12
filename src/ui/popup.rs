use std::io;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    style::ContentStyle,
};

use crate::{
    canvas::{CanvasLike, CanvasLikeExt},
    cell::Cell,
    drawable::{dbox::Dbox, extended_impl::Stylable, Drawable},
    frame::Frame,
    helpers::line_center,
    layout::{Dims, Pos},
    renderer::Renderer,
};

use super::Window;

pub fn popup_size(title: &str, texts: &[&str]) -> Dims {
    match texts.iter().map(|text| text.len()).max() {
        Some(l) => (
            2 + 2 + l.max(title.len()) as i32,
            2 + 2 + texts.len() as i32,
        )
            .into(),
        None => (4 + title.len() as i32, 3).into(),
    }
}

pub struct Popup {
    pub title: String,
    pub texts: Vec<String>,
    pub box_style: ContentStyle,
    pub text_style: ContentStyle,
}

impl Popup {
    pub fn new<T, S, TS>(title: T, texts: TS) -> Self
    where
        T: Into<String>,
        S: Into<String>,
        TS: IntoIterator<Item = S>,
    {
        Self {
            title: title.into(),
            texts: texts.into_iter().map(|s| s.into()).collect(),
            box_style: ContentStyle::default(),
            text_style: ContentStyle::default(),
        }
    }

    fn size(&self) -> Dims {
        match self.texts.iter().map(|text| text.chars().count()).max() {
            Some(l) => (
                2 + 2 + l.max(self.title.len()) as i32,
                2 + 2 + self.texts.len() as i32,
            )
                .into(),
            None => (4 + self.title.len() as i32, 3).into(),
        }
    }

    pub fn with_box_style(mut self, style: ContentStyle) -> Self {
        self.box_style = style;
        self
    }

    pub fn with_text_style(mut self, style: ContentStyle) -> Self {
        self.text_style = style;
        self
    }
}

// We impl for ref because we don't want to move the popup after each draw
impl Drawable for &mut Popup {
    type X = ();
    type Y = ();

    fn draw(self, _: impl Into<Pos<(), ()>>, frame: &mut impl CanvasLike) {
        fn draw_inner(
            title: &str,
            texts: &[String],
            box_style: ContentStyle,
            text_style: ContentStyle,
            size: Dims,
            mut frame: Frame,
        ) -> () {
            let box_size = size;
            let title_pos = line_center(0, box_size.x - 2, title.len() as i32 + 2);

            frame.fill(Cell::new(' '));

            let mut frame = Frame::new(frame).centered(box_size);
            let mut inner = frame.clone().mx(1).my(1);

            frame.draw((0, 0), Dbox::new(box_size).styled(box_style));
            inner.draw((title_pos, 0), format!(" {} ", title).styled(text_style));

            if !texts.is_empty() {
                inner.draw((0, 1), "─".repeat(box_size.x as usize - 2));
                for (i, text) in texts.iter().enumerate() {
                    inner.draw((1, i as i32 + 2), text.styled(text_style))
                }
            }
        }

        draw_inner(
            &self.title,
            &self.texts,
            self.box_style,
            self.text_style,
            self.size(),
            Frame::new(frame),
        );
    }
}

impl Window for Popup {
    type Output = io::Result<KeyCode>;

    fn run(&mut self, renderer: &mut Renderer) -> Self::Output {
        loop {
            renderer.get_render_space().draw((), &mut *self);
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
