use crossterm::event::{self, Event, KeyCode, KeyEvent};
use fyodor::{canvas::CanvasLikeExt, frame::Frame, renderer::Renderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new()?;
    let size = (10, 5);

    let mut frame_clip = Frame::new(renderer.get_render_space())
        .with_size(size)
        .with_pos((10, 5));

    let mut frame = Frame::new(renderer.get_render_space())
        .with_size(size)
        .with_pos((10, 20))
        .no_clip();

    loop {
        let event = event::read()?;

        let mut canvas = renderer.canvas();

        renderer.on_event(&event)?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind,
                ..
            }) if kind != crossterm::event::KeyEventKind::Release => break,
            _ => {}
        }

        canvas.show((0, 0), &"Press Enter to exit");
        canvas.show(
            (0, 1),
            &"Both frame are same size, just offseted and only one is clipped",
        );

        canvas.show((0, 4), &"Clipped frame");
        for x in -10..frame_clip.size.x {
            for y in -5..frame_clip.size.y {
                frame_clip.show((x, y), &"█");
            }
        }

        canvas.show((0, 19), &"Unclipped frame");
        for x in -10..frame.size.x {
            for y in -5..frame.size.y {
                frame.show((x, y), &"█");
            }
        }

        renderer.render()?;
    }

    Ok(())
}
