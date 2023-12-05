use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, ContentStyle},
};
use fyodor::{
    layout::align::{Align, Aligned},
    renderer::Renderer,
    ui::menu::Menu,
    CanvasLikeExt,
};

use std::io;

fn new_foreground(color: Color) -> ContentStyle {
    ContentStyle {
        foreground_color: Some(color),
        ..ContentStyle::default()
    }
}

fn main() -> io::Result<()> {
    let mut renderer = Renderer::new()?;

    let mut menu = Menu::new("Menu".to_string()).with_items(vec!["Item 1", "Item 2", "Item 3"]);
    // let mut menu = Menu::new("Menu".to_string()).with_items(vec![
    //     Menu::new("Submenu 1".to_string()).with_items(vec!["Item 1 1", "Item 1 2"]),
    //     Menu::new("Submenu 2".to_string()).with_items(vec!["Item 2 1", "Item 2 2"]),
    // ]);
    let pink = (227, 166, 211);
    menu.box_style = new_foreground(Color::Red);
    menu.text_style = new_foreground(Color::DarkMagenta);
    menu.item_style = new_foreground(pink.into());
    menu.selected_style = Some(ContentStyle {
        foreground_color: Some(Color::Black),
        background_color: Some(pink.into()),
        ..Default::default()
    });

    let mut menu = Aligned(menu);

    let selected = loop {
        let mut canvas = renderer.canvas();

        canvas.show((Align::Center, Align::Center), &menu);

        renderer.render()?;

        let event = crossterm::event::read()?;

        renderer.on_event(&event)?;
        match event {
            Event::Key(KeyEvent {
                code,
                kind,
                modifiers,
                ..
            }) if kind != KeyEventKind::Release => match code {
                KeyCode::Char('w' | 'W') | KeyCode::Up => {
                    menu.0.up(1);
                }
                KeyCode::Char('s' | 'S') | KeyCode::Down => {
                    menu.0.down(1);
                }
                KeyCode::Char('q' | 'Q') => {
                    break None;
                }
                KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => {
                    break None;
                }
                KeyCode::Enter => {
                    break Some(menu.0.selected().unwrap());
                }
                _ => {}
            },
            _ => {}
        }
    };

    drop(renderer);

    println!("Selected: {:?}", selected);

    Ok(())
}
