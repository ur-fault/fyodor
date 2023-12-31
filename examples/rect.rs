use fyodor::{renderer::Renderer, CanvasLikeExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new()?;

    const SIZE: (i32, i32) = (20, 10);

    loop {
        let event = crossterm::event::read()?;

        renderer.on_event(&event)?;
        match event {
            crossterm::event::Event::Key(crossterm::event::KeyEvent {
                code: crossterm::event::KeyCode::Enter,
                kind,
                ..
            }) if kind != crossterm::event::KeyEventKind::Release => break,
            _ => {}
        }

        for x in 0..SIZE.0 {
            for y in 0..SIZE.1 {
                renderer.canvas().show((x, y), &'█');
            }
        }

        renderer
            .canvas()
            .show((0, SIZE.1), &"Press Enter to exit");
        renderer.render()?;
    }

    Ok(())
}
