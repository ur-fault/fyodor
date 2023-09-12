use std::io;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    style::ContentStyle,
};

use crate::{
    drawable::{dbox::Dbox, extended_impl::Stylable},
    frame::Frame,
    helpers::{box_center_screen, line_center},
    layout::Dims,
    renderer::Renderer, canvas::CanvasLikeExt,
};

pub struct Popup<T: AsRef<str>, S: AsRef<str>, TS: IntoIterator<Item = S>> {
    pub title: T,
    pub texts: TS,
}

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

pub fn popup(
    renderer: &mut Renderer,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    texts: &[&str],
) -> io::Result<KeyCode> {
    render_popup(renderer, box_style, text_style, title, texts)?;

    loop {
        let event = read()?;
        if let Event::Key(KeyEvent { code, kind, .. }) = event {
            if kind != KeyEventKind::Release {
                break Ok(code);
            }
        }

        renderer.on_event(&event)?;

        render_popup(renderer, box_style, text_style, title, texts)?;
    }
}

pub fn render_popup(
    renderer: &mut Renderer,
    box_style: ContentStyle,
    text_style: ContentStyle,
    title: &str,
    texts: &[&str],
) -> io::Result<()> {
    let box_size = popup_size(title, texts);
    let title_pos = line_center(0, box_size.x - 2, title.len() as i32 + 2);
    let pos = box_center_screen(box_size)?;

    let mut frame = Frame::new(renderer.get_render_space())
        .with_pos(pos)
        .with_size(box_size)
        .no_clip();

    let mut inner = frame.clone().mx(1).my(1);

    frame.draw((0, 0), Dbox::new(box_size).styled(box_style));
    inner.draw((title_pos, 0), format!(" {} ", title).styled(text_style));

    if !texts.is_empty() {
        inner.draw((0, 1), "─".repeat(box_size.x as usize - 2));
        for (i, text) in texts.iter().enumerate() {
            inner.draw((1, i as i32 + 2), text.styled(text_style))
        }
    }

    renderer.render()?;

    Ok(())
}
