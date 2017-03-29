use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;

// Constants
const PLAYER_SPEED: f64 = 180.0;
// Data types

// View definition
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn to_sdl(self) -> Option<SdlRect> {
        assert!(self.w >= 0.0 && self.h >= 0.0);
        Some(SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32))
    }
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        if self.w > parent.w || self.h > parent.h {
            return None;
        }
        Some(Rectangle {
                 w: self.w,
                 h: self.h,
                 x: if self.x < parent.x {
                     parent.x
                 } else if self.x + self.w >= parent.x + parent.w {
            parent.x + parent.w - self.w
        } else {
            self.x
        },
                 y: if self.y < parent.y {
                     parent.y
                 } else if self.y + self.h >= parent.y + parent.h {
            parent.y + parent.h - self.h
        } else {
            self.y
        },
             })

    }
}
struct Ship {
    rect: Rectangle,
}
pub struct ShipView {
    player: Ship,
}
impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                },
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

        phi.renderer.fill_rect(self.player
                                   .rect
                                   .move_inside(movable_region)
                                   .unwrap()
                                   .to_sdl()
                                   .unwrap());

        ViewAction::None
    }
}
