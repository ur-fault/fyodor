use std::{
    io::{self},
    iter,
};

use fyodor::{
    renderer::Renderer,
    ui::{popup::Popup, Window},
};
use lipsum::{lipsum_title, lipsum_words_with_rng};
use rand::thread_rng;

fn main() -> io::Result<()> {
    let mut rng = thread_rng();
    const COUNT: usize = 10;
    let lines = iter::from_fn(|| Some(lipsum_words_with_rng(&mut rng, 5)))
        .take(COUNT)
        .collect::<Vec<_>>();

    let mut renderer = Renderer::new()?;

    Popup::new(lipsum_title(), lines).run(&mut renderer)?;

    Ok(())
}
