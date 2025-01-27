use std::time::Duration;

use smithay::backend::renderer::element::RenderElement;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::utils::{Physical, Size};

pub mod layout;
pub mod tile;
pub mod window;

pub trait TestCase {
    fn resize(&mut self, width: i32, height: i32);
    fn are_animations_ongoing(&self) -> bool {
        false
    }
    fn advance_animations(&mut self, _current_time: Duration) {}
    fn render(
        &mut self,
        renderer: &mut GlesRenderer,
        size: Size<i32, Physical>,
    ) -> Vec<Box<dyn RenderElement<GlesRenderer>>>;
}
