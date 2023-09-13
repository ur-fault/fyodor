use crossterm::style::{Color, ContentStyle};
use fyodor::{
    renderer::Renderer,
    ui::{popup::Popup, Window},
};

use std::io;

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;

    let mut style = ContentStyle::new();
    style.foreground_color = Some(Color::Red);
    Popup::new("Popup title", ["Popup text", "Popup text 2", "Long Long Popup text"])
        .with_text_style(style)
        .run(&mut renderer)?;

    Ok(())
}
