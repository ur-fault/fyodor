use crossterm::style::ContentStyle;

use fyodor::{renderer::Renderer, ui::popup::popup};

use std::io;

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;

    popup(
        &mut renderer,
        ContentStyle::default(),
        ContentStyle::default(),
        "Popup title",
        &["Popup text", "Popup text 2"],
    )?;

    Ok(())
}
