use std::io;

use crate::renderer::Renderer;

pub mod popup;
pub mod fullscreen_popup;
pub mod menu;
pub mod fullscreen_menu;

pub trait Window {
    type Output<'a> where Self: 'a;

    fn run(&mut self, renderer: &mut Renderer) -> io::Result<Self::Output<'_>>;
}
