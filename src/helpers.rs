use std::io;

use crate::layout::{Dims, Pos};

pub fn term_size() -> Dims {
    let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
    Pos::new(w as i32, h as i32)
}

pub fn line_center(container_start: i32, container_end: i32, item_width: i32) -> i32 {
    (container_end - container_start - item_width) / 2 + container_start
}

pub fn box_center(container_start: Dims, container_end: Dims, box_dims: Dims) -> Dims {
    (container_end - container_start - box_dims) / Pos::sq(2) + container_start
}

pub fn box_center_screen(box_dims: Dims) -> io::Result<Dims> {
    let size_u16 = term_size();
    Ok(box_center(
        (0, 0).into(),
        (size_u16.x as i32, size_u16.y as i32).into(),
        box_dims,
    ))
}
