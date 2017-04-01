extern crate sdl2;

#[macro_use]
mod phi;
mod views;



fn main() {
    // Initialize SDL2
    ::phi::spawn("ArcadeRS Shooter",
                 |phi| Box::new(views::game::ShipView::new(phi)));
                 
  /*  ::phi::spawn("ArcadeRS Shooter",
                 |phi| Box::new(views::main_menu::MainMenuView::new(phi)));
                 */
}
