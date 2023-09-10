use crossterm::event::{self, Event, KeyCode, KeyEvent};
use terminal_renderer::{canvas::CanvasLikeExt, frame::Frame, renderer::Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new()?;
    const SIZE: (i32, i32) = (10, 5);

    let mut frame_clip = Frame::new(renderer.get_render_space())
        .with_size(SIZE)
        .with_pos((10, 5));

    let mut frame = Frame::new(renderer.get_render_space())
        .with_size(SIZE)
        .with_pos((10, 20))
        .no_clip();

    loop {
        let event = event::read()?;

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
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
                frame_clip.draw((x, y), "█");
            }
        }
        renderer.canvas().draw((0, 4), "Clipped frame");

        for x in -10..frame.size.0 {
            for y in -5..frame.size.1 {
                frame.draw((x, y), "█");
            }
        }
        renderer.canvas().draw((0, 19), "Unclipped frame");

        renderer.render()?;
    }

    Ok(())
}
