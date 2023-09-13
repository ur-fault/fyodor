use crate::renderer::Renderer;

pub mod popup;

pub trait Window {
    type Output;

    fn run(&mut self, renderer: &mut Renderer) -> Self::Output;
}
