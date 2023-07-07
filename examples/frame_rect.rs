use terminal_renderer::{drawable::Drawable, frame::FrameBuilder, renderer::Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new()?;
    const SIZE: (i32, i32) = (10, 5);

    let frame_builder_clip = FrameBuilder::new().with_size(SIZE).with_pos((10, 5));
    let frame_builder = FrameBuilder::new()
        .with_size(SIZE)
        .with_pos((10, 20))
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

        let mut frame = frame_builder_clip.clone().build(renderer.canvas());
        for x in -10..frame.size.0 {
            for y in -5..frame.size.1 {
                // frame.draw((x, y), "█");
                '█'.draw((x, y), &mut frame);
            }
        }
        renderer.canvas().draw((0, 4), "Clipped frame");

        let mut frame = frame_builder.clone().build(renderer.canvas());
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
