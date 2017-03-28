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
    let mut timer = sdl_context.timer().unwrap();

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
    let interval = 1_000/60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    // Prepare the events record

    loop {
        let now = timer.ticks();
        let dt = now - before;
    let elapsed = dt as f64 / 1_000.0;
        if dt < interval {
            timer.delay(interval - dt );
            continue
        }
        before = now;
        fps+=1;
        if now - last_second > 1_000{
            println!("FPS: {}", fps);
            last_second = now;
            fps=0;
        }
        
        context.events.pump();
        match current_view.render(&mut context,elapsed){
            ViewAction::None=>context.renderer.present(),
            ViewAction::Quit=>break,
        }
    }


}
