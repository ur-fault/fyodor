use crossterm::style::{Color, ContentStyle};

use crate::{
    canvas::CanvasLike,
    drawable::{dbox::Dbox, styled::Stylable, Drawable},
    layout::{
        align::{Align, AlignedOnX},
        sized::{FullyKnown, KnownHeight, KnownWidth},
        Pos,
    },
    CanvasLikeExt, Frame,
};

fn flip_fg_bg(style: ContentStyle) -> ContentStyle {
    ContentStyle {
        background_color: Some(style.foreground_color.unwrap_or(Color::White)),
        foreground_color: Some(style.background_color.unwrap_or(Color::Black)),
        underline_color: None,
        attributes: Default::default(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Menu<T> {
    title: String,
    items: Vec<T>,
    pub numbered: bool,
    selected: usize,
    pub box_style: ContentStyle,
    pub text_style: ContentStyle,
    pub item_style: ContentStyle,
    pub selected_style: Option<ContentStyle>,
}

impl<T> Menu<T> {
    pub fn new(title: String) -> Self {
        // TODO: allow Into<String>
        Self {
            title,
            items: Vec::new(),
            numbered: false,
            selected: 0,
            box_style: ContentStyle::default(),
            text_style: ContentStyle::default(),
            item_style: ContentStyle::default(),
            selected_style: None,
        }
    }

    pub fn with_items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    pub fn items(&self) -> &[T] {
        self.items.as_ref()
    }
}

impl<T> Menu<T> {
    pub fn selected(&self) -> Option<&T> {
        self.items.get(self.selected)
    }

    pub fn selected_index(&self) -> Option<usize> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.selected)
        }
    }

    pub fn select(&mut self, i: usize) {
        self.selected = i.clamp(0, self.items.len() - 1);
    }

    pub fn first(&mut self) {
        self.selected = 0;
    }

    pub fn last(&mut self) {
        self.selected = self.items.len() - 1;
    }

    pub fn up(&mut self, c: usize) {
        let c = c % self.items.len();
        let sel = self.selected as i64 - c as i64;
        if sel < 0 {
            self.selected = self.items.len() - (-sel as usize);
        } else {
            self.selected = sel as usize;
        }
    }

    pub fn down(&mut self, c: usize) {
        let c = c % self.items.len();
        self.selected = (self.selected + c) % self.items.len();
    }
}

impl<T> Drawable for Menu<T>
where
    T: FullyKnown + Stylable,
    for<'a> (ContentStyle, &'a T): Drawable<X = i32, Y = i32>,
{
    type X = i32;
    type Y = i32;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        // TODO: make draw_inner that isn't generic
        let size = self.dims();

        let pos: Pos<i32, i32> = pos.into();

        let mut frame = Frame::new(frame).with_size(size).with_pos(pos);

        Dbox::new(size)
            .styled(self.box_style)
            .draw((0, 0), &mut frame);

        frame.show(
            (1, 2),
            &"─".repeat(size.x as usize - 2).styled(self.box_style),
        );

        let title = format!(" {} ", self.title);

        frame.show(
            (Align::Center, 1),
            &AlignedOnX(title.as_str()).styled(self.text_style),
        );

        let mut y = 3;
        for (i, item) in self.items.iter().enumerate() {
            let selected = self.selected == i;

            let style = if selected {
                self.selected_style.unwrap_or(self.item_style)
            } else {
                self.item_style
            };

            let h = item.h();
            let item = item.styled(style);

            if selected {
                frame.show((1, y), &"> ".styled(style));
            }

            if self.numbered {
                frame.show((3, y), &format!("{}. ", i + 1).styled(style));
            }

            let numbered_len = if self.numbered {
                (self.items.len() + 1).to_string().len() as i32 + 2
            } else {
                0
            };

            frame.show((numbered_len + 3, y), &item);

            y += h;
        }
    }
}

impl<T> Drawable for (ContentStyle, &Menu<T>)
where
    Menu<T>: Clone + Drawable,
{
    type X = <Menu<T> as Drawable>::X;
    type Y = <Menu<T> as Drawable>::Y;

    fn draw(&self, pos: impl Into<Pos<Self::X, Self::Y>>, frame: &mut impl CanvasLike) {
        let mut menu = self.1.clone();
        menu.box_style = self.0;
        menu.text_style = self.0;
        menu.item_style = self.0;
        menu.selected_style = Some(flip_fg_bg(self.0));

        menu.draw(pos, frame);
    }
}

impl<T> KnownWidth for Menu<T>
where
    T: KnownWidth,
{
    fn w(&self) -> i32 {
        2 + 2 // box + "> "
            + if let Some(longest_item) = self.items.iter().map(|opt| opt.w()).max() {
                (if self.numbered {
                    (self.items.len() + 1).to_string().len() as i32 + 2 // longest number + ". "
                } else {
                    0
                } + longest_item)
            } else {
                0
            }
            .max(self.title.len() as i32 + 2) // title + spaces
    }
}

impl<T> KnownHeight for Menu<T>
where
    T: KnownHeight,
{
    fn h(&self) -> i32 {
        self.items.iter().map(|i| i.h()).sum::<i32>() + 2 + 2
    }
}
