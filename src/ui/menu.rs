use crossterm::style::ContentStyle;

use crate::{
    canvas::CanvasLike,
    drawable::Drawable,
    layout::{sized::Align, Pos},
};

pub struct Menu<T> {
    title: String,
    items: Vec<T>,
    numbered: bool,
    selected: usize,
    box_style: ContentStyle,
    text_style: ContentStyle,
    selected_style: Option<ContentStyle>,
}

impl<T> Menu<T> {
    pub fn new(title: String) -> Self {
        Self {
            title,
            items: Vec::new(),
            numbered: false,
            selected: 0,
            box_style: ContentStyle::default(),
            text_style: ContentStyle::default(),
            selected_style: None,
        }
    }

    pub fn with_items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }
}

impl<T> Drawable for Menu<T>
where
    T: Into<String>,
{
    type X = Align;
    type Y = Align;

    fn draw(self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        todo!()
    }
}
