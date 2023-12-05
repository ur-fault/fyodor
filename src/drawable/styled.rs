use crossterm::style::ContentStyle;

use crate::Drawable;

pub trait Stylable: Drawable {
    fn styled(&self, style: ContentStyle) -> (ContentStyle, &Self);
}

impl<D> Stylable for D
where
    D: Drawable + ?Sized,
    for<'a> (ContentStyle, &'a D): Drawable,
{
    fn styled(&self, style: ContentStyle) -> (ContentStyle, &Self) {
        (style, self)
    }
}
