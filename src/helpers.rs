use std::io;

use crate::renderer::Dims;

pub fn term_size() -> Dims {
    let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
    (w as i32, h as i32)
}

pub fn line_center(container_start: i32, container_end: i32, item_width: i32) -> i32 {
    (container_end - container_start - item_width) / 2 + container_start
}

pub fn box_center(container_start: Dims, container_end: Dims, box_dims: Dims) -> Dims {
    (
        line_center(container_start.0, container_end.0, box_dims.0),
        line_center(container_start.1, container_end.1, box_dims.1),
    )
}

pub fn box_center_screen(box_dims: Dims) -> io::Result<Dims> {
    let size_u16 = term_size();
    Ok(box_center(
        (0, 0),
        (size_u16.0 as i32, size_u16.1 as i32),
        box_dims,
    ))
}
