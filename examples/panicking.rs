use std::io::{self};

use fyodor::renderer::Renderer;

#[allow(unreachable_code)]
fn main() -> io::Result<()> {
    // is neaded because of the handler
    let _renderer = Renderer::new()?;

    panic!("test");

    Ok(())
}
