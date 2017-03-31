use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;
use phi::data::Rectangle;
use phi::gfx::Sprite;
use std::path::Path;
use sdl2::render::{Texture, TextureQuery};
use sdl2::image::LoadTexture;

// Constants
const PLAYER_SPEED: f64 = 180.0;

struct Ship {
    rect: Rectangle,
    sprite: Sprite,
}
pub struct ShipView {
    player: Ship,
}
impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        let sprite = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                },
                sprite: sprite,
            },
        }
    }
}
impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // [TODO] Insert the moving logic here
        let traveled = PLAYER_SPEED * elapsed;
        let diagonal = (phi.events.key_up ^ phi.events.key_down) &&
                       (phi.events.key_left ^ phi.events.key_right);
        let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;
        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };
        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };
        self.player.rect.x += dx;
        self.player.rect.y += dy;
        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };
        self.player.rect = self.player
            .rect
            .move_inside(movable_region)
            .unwrap();
        self.player.sprite.render(&mut phi.renderer, self.player.rect);
        ViewAction::None
    }
}
