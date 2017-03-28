extern crate sdl2;

use sdl2::pixels::Color;


#[macro_use]
mod phi;
mod views;



fn main() {
    // Initialize SDL2
    ::phi::spawn("ArcadeRS Shooter",
                 |phi| Box::new(views::ShipView::new(phi)));

}
