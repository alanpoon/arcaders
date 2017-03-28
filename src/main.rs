extern crate sdl2;

use sdl2::pixels::Color;


#[macro_use]
mod phi;
mod views;
use phi::{Events, Phi,View,ViewAction};


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
    // Create Phi Context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().unwrap()),
        renderer: window.renderer()
            .accelerated()
            .build()
            .unwrap(),
    };
    //create default view using a box
    let mut current_view:Box<View> = Box::new(views::DefaultView);
    // Prepare the events record

    loop {
        context.events.pump();
        match current_view.render(&mut context,0.01){
            ViewAction::None=>context.renderer.present(),
            ViewAction::Quit=>break,
        }
    }


}
