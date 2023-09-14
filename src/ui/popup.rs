use crossterm::style::ContentStyle;

use crate::{
    canvas::{CanvasLike, CanvasLikeExt},
    drawable::{dbox::Dbox, extended_impl::Stylable, Drawable},
    frame::Frame,
    layout::{
        axis::Axis,
        sized::{Aligned, Anchor, KnownHeight, KnownWidth},
        Dims, Pos,
    },
};

use super::fullscreen_popup::FullScreenPopup;

pub struct Popup {
    title: String,
    texts: Option<Vec<String>>,
    pub box_style: ContentStyle,
    pub text_style: ContentStyle,
}

impl Popup {
    pub fn new<S>(title: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            title: title.into(),
            texts: None,
            box_style: ContentStyle::default(),
            text_style: ContentStyle::default(),
        }
    }

    pub fn with_texts<S, TS>(mut self, texts: TS) -> Self
    where
        S: Into<String>,
        TS: IntoIterator<Item = S>,
    {
        self.texts = Some(texts.into_iter().map(Into::into).collect());
        self
    }

    fn size(&self) -> Dims {
        let width = match self.texts {
            Some(ref texts) => {
                texts
                    .iter()
                    .map(|t| t.chars().count())
                    .max()
                    .unwrap_or(0) // longest of texts
                    .max(self.title.chars().count()) as i32
                    + 2
                    + 2
            }
            None => 2 + 2 + self.title.chars().count() as i32,
        };

        let height = match self.texts {
            Some(ref texts) => 2 + 2 + texts.len() as i32,
            None => 3,
        };

        Pos::new(width, height)
    }

    pub fn with_box_style(mut self, style: ContentStyle) -> Self {
        self.box_style = style;
        self
    }

    pub fn with_text_style(mut self, style: ContentStyle) -> Self {
        self.text_style = style;
        self
    }

    pub fn to_window(self) -> FullScreenPopup {
        FullScreenPopup::new(self)
    }
}

// We impl for ref because we don't want to move the popup after each draw
impl Drawable for &mut Popup {
    type X = Aligned;
    type Y = Aligned;

    fn draw(self, pos: impl Into<Pos<Aligned, Aligned>>, frame: &mut impl CanvasLike) {
        fn draw_inner(
            title: &str,
            texts: Option<&Vec<String>>,
            box_style: ContentStyle,
            text_style: ContentStyle,
            Pos { x, y }: Dims,
            box_size: Dims,
            frame: Frame,
        ) -> () {
            let mut frame = Frame::new(frame).ml(x).mt(y).with_size(box_size);
            let mut inner = frame.clone().mx(1).my(1);

            let title_pos = Aligned::new(Anchor::Center, title.w() + 2).calc(box_size.x - 2);

            frame.draw((0, 0), Dbox::new(box_size).styled(box_style));
            inner.draw((title_pos, 0), format!(" {} ", title).styled(text_style));

            if let Some(texts) = texts {
                inner.draw((0, 1), "─".repeat(box_size.x as usize - 2));
                for (i, text) in texts.iter().enumerate() {
                    inner.draw((1, i as i32 + 2), text.styled(text_style))
                }
            }
        }

        let Pos { x, y } = pos.into();

        draw_inner(
            &self.title,
            self.texts.as_ref(),
            self.box_style,
            self.text_style,
            Pos::new(x.calc(frame.w()), y.calc(frame.h())),
            self.size(),
            Frame::new(frame),
        );
    }
}

impl KnownWidth for Popup {
    fn w(&self) -> i32 {
        self.size().x
    }
}

impl KnownHeight for Popup {
    fn h(&self) -> i32 {
        self.size().y
    }
}
