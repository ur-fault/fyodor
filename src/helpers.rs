use crate::renderer::Dims;

pub fn term_size() -> Dims {
    let (w, h) = crossterm::terminal::size().unwrap_or((80, 24));
    (w as i32, h as i32)
}
