extern crate sdl2;
extern crate rand;

#[macro_use]
mod phi;
mod views;
use views::shared::Background;


fn main() {
    // Initialize SDL2
    ::phi::spawn("ArcadeRS Shooter", |phi| {
        let bg = ::views::shared::BgSet::new(&mut phi.renderer);
        Box::new(::views::game::GameView::with_backgrounds(phi, bg))
    });

}
