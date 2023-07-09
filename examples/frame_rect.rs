use terminal_renderer::{drawable::Drawable, frame::Frame, renderer::Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new()?;
    const SIZE: (i32, i32) = (10, 5);

    let mut frame_clip = Frame::new(renderer.canvas())
        .with_size(SIZE)
        .with_pos((10, 5));
    let mut frame = Frame::new(renderer.canvas())
        .with_size(SIZE)
        .with_pos((10, 5))
        .no_clip();

    loop {
        let event = crossterm::event::read()?;

        match event {
            crossterm::event::Event::Key(crossterm::event::KeyEvent {
                code: crossterm::event::KeyCode::Enter,
                kind,
                ..
            }) if kind != crossterm::event::KeyEventKind::Release => break,
            _ => {}
        }

        renderer.canvas().draw((0, 0), "Press Enter to exit");
        renderer.canvas().draw(
            (0, 1),
            "Both frame are same size, just offseted and only one is clipped",
        );
        for x in -10..frame_clip.size.0 {
            for y in -5..frame_clip.size.1 {
                // frame.draw((x, y), "█");
                '█'.draw((x, y), &mut frame_clip);
            }
        }
        renderer.canvas().draw((0, 4), "Clipped frame");

        for x in -10..frame.size.0 {
            for y in -5..frame.size.1 {
                // frame.draw((x, y), "█");
                '█'.draw((x, y), &mut frame);
            }
        }
        renderer.canvas().draw((0, 19), "Unclipped frame");

        renderer.render()?;
    }

    Ok(())
}
