#[macro_use]
mod events;

use sdl2::render::Renderer;


struct_events! [
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
]
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}


pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
