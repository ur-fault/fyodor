use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::{Color, ContentStyle},
};
use fyodor::{
    layout::align::{Align, Aligned},
    renderer::Renderer,
    ui::{
        fullscreen_menu::FullscreenMenu, fullscreen_popup::FullScreenPopup, menu::Menu,
        popup::Popup, Window,
    },
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
    menu.box_style = new_foreground(pink.into());
    menu.text_style = new_foreground(pink.into());
    menu.item_style = new_foreground(pink.into());
    menu.selected_style = Some(ContentStyle {
        foreground_color: Some(Color::Black),
        background_color: Some(pink.into()),
        ..Default::default()
    });

    let mut menu = FullscreenMenu::new(menu);

    let selected = menu.run(&mut renderer)?;

    FullScreenPopup::new(Popup::new("Selected item").with_texts(vec![format!("{:?}", selected)]))
        .run(&mut renderer)?;

    drop(renderer);

    Ok(())
}
