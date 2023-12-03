use std::io;

use crate::layout::Dims;

pub fn term_size() -> Dims {
    let (w, h) = crossterm::terminal::size().unwrap_or((80, 24)); // mainly cuz of docker
    Dims::new(w as i32, h as i32)
}

pub fn line_center(container_start: i32, container_end: i32, item_width: i32) -> i32 {
    (container_end - container_start - item_width) / 2 + container_start
}

pub fn box_center(
    container_start: impl Into<Dims>,
    container_end: impl Into<Dims>,
    box_dims: impl Into<Dims>,
) -> Dims {
    let container_start = container_start.into();
    (container_end.into() - container_start - box_dims.into()) / 2 + container_start
}

pub fn box_center_screen(box_dims: impl Into<Dims>) -> io::Result<Dims> {
    Ok(box_center((0, 0), term_size(), box_dims))
}
