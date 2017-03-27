extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;

#[macro_use]
mod events;
struct_events![
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
]
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
fn main() {
    // Initialize SDL2
    let mut sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());
    loop {
        events.pump();
        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }


}
