use phi::{Phi, View, ViewAction};
use phi::gfx::{CopySprite, Sprite, AnimatedSprite, AnimatedSpriteDescr};
use phi::data::{Rectangle, MaybeAlive};
use std::path::Path;
use sdl2::render::{Texture, TextureQuery};
use sdl2::image::LoadTexture;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use views::shared::{Background, BgSet};

// Constants
const PLAYER_SPEED: f64 = 180.0;
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;
const DEBUG: bool = false;
const ASTEROID_PATH: &'static str = "assets/asteroid.png";
const ASTEROIDS_WIDE: usize = 21;
const ASTEROIDS_HIGH: usize = 7;
const ASTEROIDS_TOTAL: usize = ASTEROIDS_WIDE * ASTEROIDS_HIGH - 4;
const ASTEROID_SIDE: f64 = 96.0;
const BULLET_SPEED: f64 = 240.0;
const BULLET_W: f64 = 8.0;
const BULLET_H: f64 = 4.0;
const EXPLOSION_PATH: &'static str = "assets/explosion.png";
const EXPLOSIONS_WIDE: usize = 5;
const EXPLOSIONS_HIGH: usize = 4;
const EXPLOSIONS_TOTAL: usize = 17;
const EXPLOSION_SIDE: f64 = 96.0;
const EXPLOSION_FPS: f64 = 16.0;
const EXPLOSION_DURATION: f64 = 1.0 / EXPLOSION_FPS * EXPLOSIONS_TOTAL as f64;

struct Asteroid {
    sprite: AnimatedSprite,
    rect: Rectangle,
    vel: f64,
}

impl Asteroid {
    fn factory(phi: &mut Phi) -> AsteroidFactory {
        AsteroidFactory {
            sprite: AnimatedSprite::with_fps(AnimatedSprite::load_frames(phi,
                                                                         AnimatedSpriteDescr {
                                                                             image_path:
                                                                                 ASTEROID_PATH,
                                                                             total_frames:
                                                                                 ASTEROIDS_TOTAL,
                                                                             frames_high:
                                                                                 ASTEROIDS_HIGH,
                                                                             frames_wide:
                                                                                 ASTEROIDS_WIDE,
                                                                             frame_w: ASTEROID_SIDE,
                                                                             frame_h: ASTEROID_SIDE,
                                                                         }),
                                             1.0),
        }
    }
    fn new(phi: &mut Phi) -> Asteroid {
        let mut asteroid = Asteroid {
            sprite: Asteroid::get_sprite(phi, 1.0),
            rect: Rectangle {
                w: ASTEROID_SIDE,
                h: ASTEROID_SIDE,
                x: 128.0,
                y: 128.0,
            },
            vel: 0.0,
        };
        asteroid.reset(phi);
        asteroid
    }
    fn reset(&mut self, phi: &mut Phi) {
        let (w, h) = phi.output_size();
        self.sprite.set_fps(::rand::random::<f64>().abs() * 20.0 + 10.0);
        self.rect = Rectangle {
            w: ASTEROID_SIDE,
            h: ASTEROID_SIDE,
            x: w,
            y: ::rand::random::<f64>().abs() * (h - ASTEROID_SIDE),
        };
        self.vel = ::rand::random::<f64>().abs() * 100.0 + 50.0;
    }
    fn get_sprite(phi: &mut Phi, fps: f64) -> AnimatedSprite {
        let asteroid_spritesheet = Sprite::load(&mut phi.renderer, ASTEROID_PATH).unwrap();
        let mut asteroid_sprites = Vec::with_capacity(ASTEROIDS_TOTAL);
        for yth in 0..ASTEROIDS_HIGH {
            for xth in 0..ASTEROIDS_WIDE {
                if ASTEROIDS_WIDE * yth + xth >= ASTEROIDS_TOTAL {
                    break;
                }
                asteroid_sprites.push(asteroid_spritesheet.region(Rectangle {
                                                                      x: ASTEROID_SIDE * xth as f64,
                                                                      y: ASTEROID_SIDE * yth as f64,
                                                                      w: ASTEROID_SIDE,
                                                                      h: ASTEROID_SIDE,
                                                                  })
                                          .unwrap())
            }
        }
        AnimatedSprite::new(asteroid_sprites, fps)
    }
    fn update(mut self, dt: f64) -> Option<Asteroid> {
        self.rect.x -= dt * self.vel;
        self.sprite.add_time(dt);

        if self.rect.x <= -ASTEROID_SIDE {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        if DEBUG {
            // Render the bounding box
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
        }
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }
    fn rect(&self) -> Rectangle {
        self.rect
    }
}
struct AsteroidFactory {
    sprite: AnimatedSprite,
}
impl AsteroidFactory {
    fn random(&self, phi: &mut Phi) -> Asteroid {
        let (w, h) = phi.output_size();

        // FPS in [10.0, 30.0)
        let mut sprite = self.sprite.clone();
        sprite.set_fps(::rand::random::<f64>().abs() * 20.0 + 10.0);

        Asteroid {
            sprite: sprite,

            // In the screen vertically, and over the right of the screen
            // horizontally.
            rect: Rectangle {
                w: ASTEROID_SIDE,
                h: ASTEROID_SIDE,
                x: w,
                y: ::rand::random::<f64>().abs() * (h - ASTEROID_SIDE),
            },

            // vel in [50.0, 150.0)
            vel: ::rand::random::<f64>().abs() * 100.0 + 50.0,
        }
    }
}
struct Explosion {
    sprite: AnimatedSprite,
    rect: Rectangle,
    alive_since: f64,
}
impl Explosion {
    fn factory(phi: &mut Phi) -> ExplosionFactory {
        // Read the asteroid's image from the filesystem and construct an
        // animated sprite out of it.

        let explosion_spritesheet = Sprite::load(&mut phi.renderer, EXPLOSION_PATH).unwrap();
        let mut explosion_sprites = Vec::with_capacity(EXPLOSIONS_TOTAL);

        for yth in 0..EXPLOSIONS_HIGH {
            for xth in 0..EXPLOSIONS_WIDE {
                if EXPLOSIONS_WIDE * yth + xth >= EXPLOSIONS_TOTAL {
                    break;
                }

                explosion_sprites.push(explosion_spritesheet.region(Rectangle {
                                                                        w: EXPLOSION_SIDE,
                                                                        h: EXPLOSION_SIDE,
                                                                        x: EXPLOSION_SIDE *
                                                                           xth as f64,
                                                                        y: EXPLOSION_SIDE *
                                                                           yth as f64,
                                                                    })
                                           .unwrap());
            }
        }

        // Return the data required to build an asteroid
        ExplosionFactory { sprite: AnimatedSprite::with_fps(explosion_sprites, EXPLOSION_FPS) }
    }
}
struct ExplosionFactory {
    sprite: AnimatedSprite,
}
impl ExplosionFactory {
    fn at_center(&self, center: (f64, f64)) {}
}
trait Bullet {
    fn update(self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>>;
    fn render(&self, phi: &mut Phi);
    fn rect(&self) -> Rectangle;
}

struct RectBullet {
    rect: Rectangle,
}
impl Bullet for RectBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        let (w, _) = phi.output_size();
        self.rect.x += dt * BULLET_SPEED;
        if self.rect.x > w { None } else { Some(self) }
    }
    fn render(&self, phi: &mut Phi) {
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect.to_sdl().unwrap());
    }
    fn rect(&self) -> Rectangle {
        self.rect
    }
}


#[derive(Clone, Copy)]
struct SineBullet {
    pos_x: f64,
    origin_y: f64,
    amplitude: f64,
    angular_vel: f64,
    total_time: f64,
}
impl Bullet for SineBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        //? We store the total time...
        self.total_time += dt;

        //? And move at the same speed as regular bullets.
        self.pos_x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        let (w, _) = phi.output_size();

        if self.rect().x > w { None } else { Some(self) }
    }

    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow.
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        //? Just the general form of the sine function, minus the initial time.
        let dy = self.amplitude * f64::sin(self.angular_vel * self.total_time);
        Rectangle {
            x: self.pos_x,
            y: self.origin_y + dy,
            w: BULLET_W,
            h: BULLET_H,
        }
    }
}

#[derive(Clone, Copy)]
enum CannonType {
    RectBullet,
    SineBullet { amplitude: f64, angular_vel: f64 },
}
#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm = 0,
    UpFast = 1,
    UpSlow = 2,
    MidNorm = 3,
    MidFast = 4,
    MidSlow = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8,
}
struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
    cannon: CannonType,
}
impl Ship {
    fn spawn_bullets(&self) -> Vec<Box<Bullet>> {
        let cannons_x = self.rect.x + 30.0;
        let cannon1_y = self.rect.y + 6.0;
        let cannon2_y = self.rect.y + SHIP_H - 10.0;
        match self.cannon {
            CannonType::RectBullet=>
                // One bullet at the tip of every cannon
        vec![Box::new(RectBullet {
                 rect: Rectangle {
                     x: cannons_x,
                     y: cannon1_y,
                     w: BULLET_W,
                     h: BULLET_H,
                 },
             }),
             Box::new(RectBullet {
                 rect: Rectangle {
                     x: cannons_x,
                     y: cannon2_y,
                     w: BULLET_W,
                     h: BULLET_H,
                 },
             })],

            CannonType::SineBullet { amplitude, angular_vel } =>
                vec![
                    Box::new(SineBullet {
                        pos_x: cannons_x,
                        origin_y: cannon1_y,
                        amplitude: amplitude,
                        angular_vel: angular_vel,
                        total_time: 0.0,
                    }),
                    Box::new(SineBullet {
                        pos_x: cannons_x,
                        origin_y: cannon2_y,
                        amplitude: amplitude,
                        angular_vel: angular_vel,
                        total_time: 0.0,
                    }),
                ]
        }

    }
}
pub struct GameView {
    player: Ship,
    bullets: Vec<Box<Bullet>>,
    asteroids: Vec<Asteroid>,
    bg: BgSet,
    asteroid_factory: AsteroidFactory,
}
impl GameView {
    #[allow(dead_code)]
    pub fn new(phi: &mut Phi) -> GameView {
        let bg = BgSet::new(&mut phi.renderer);
        GameView::with_backgrounds(phi, bg)
    }

    pub fn with_backgrounds(phi: &mut Phi, bg: BgSet) -> GameView {
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                                                    w: SHIP_W,
                                                    h: SHIP_H,
                                                    x: SHIP_W * x as f64,
                                                    y: SHIP_H * y as f64,
                                                })
                                 .unwrap());
            }
        }

        GameView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
                cannon: CannonType::RectBullet,
            },
            bullets: vec![],
            asteroids: vec![],
            bg: bg,
            asteroid_factory: Asteroid::factory(phi),
        }
    }
}
impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.key_escape == Some(true) {
            let bg = self.bg.clone();
            return ViewAction::ChangeView(Box::new(::views::main_menu::MainMenuView::with_backgrounds(phi,bg)));
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
        // Render the Backgrounds
        self.bg.back.render(&mut phi.renderer, elapsed);
        self.bg.middle.render(&mut phi.renderer, elapsed);
        // Render the bounding box (for debugging purposes)
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player
                                       .rect
                                       .to_sdl()
                                       .unwrap());
        }

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
        self.player.current = if dx == 0.0 && dy < 0.0 {
            ShipFrame::UpNorm
        } else if dx > 0.0 && dy < 0.0 {
            ShipFrame::UpFast
        } else if dx < 0.0 && dy < 0.0 {
            ShipFrame::UpSlow
        } else if dx == 0.0 && dy == 0.0 {
            ShipFrame::MidNorm
        } else if dx > 0.0 && dy == 0.0 {
            ShipFrame::MidFast
        } else if dx < 0.0 && dy == 0.0 {
            ShipFrame::MidSlow
        } else if dx == 0.0 && dy > 0.0 {
            ShipFrame::DownNorm
        } else if dx > 0.0 && dy > 0.0 {
            ShipFrame::DownFast
        } else if dx < 0.0 && dy > 0.0 {
            ShipFrame::DownSlow
        } else {
            unreachable!()
        };
        let mut player_alive = true;
        // Update the Bullet pos
        self.bullets = ::std::mem::replace(&mut self.bullets, vec![])
            .into_iter()
            .filter_map(|bullet| bullet.update(phi, elapsed))
            .collect();
        // Update the Asteroids

        self.asteroids = ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| asteroid.update(elapsed))
            .collect();
        //can keep track of which got into a collision
        let mut transition_bullets: Vec<_> = ::std::mem::replace(&mut self.bullets, vec![])
            .into_iter()
            .map(|bullet| {
                     MaybeAlive {
                         alive: true,
                         value: bullet,
                     }
                 })
            .collect();

        self.asteroids = ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| {
                let mut asteroid_alive = true;

                for bullet in &mut transition_bullets {
                    //? Notice that we refer to the bullet as `bullet.value`
                    //? because it has been wrapped in `MaybeAlive`.
                    if asteroid.rect().overlaps(bullet.value.rect()) {
                        asteroid_alive = false;
                        //? We go through every bullet and "kill" those that collide
                        //? with the asteroid. We do this for every asteroid.
                        bullet.alive = false;
                    }
                }

                // The player's ship is destroyed if it is hit by an asteroid.
                // In which case, the asteroid is also destroyed.
                if asteroid.rect().overlaps(self.player.rect) {
                    asteroid_alive = false;
                    player_alive = false;
                }

                //? Then, we use the magic of `filter_map` to keep only the asteroids
                //? that didn't explode.
                if asteroid_alive { Some(asteroid) } else { None }
            })
            .collect();

        self.bullets = transition_bullets.into_iter().filter_map(MaybeAlive::as_option).collect();
        if !player_alive {
            println!("The player's ship has been destroyed.");
        }
        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }
        if ::rand::random::<usize>() % 100 == 0 {
            self.asteroids.push(self.asteroid_factory.random(phi));
        }
        println!("{}", self.asteroids.len());

        if phi.events.now.key_1 == Some(true) {
            self.player.cannon = CannonType::RectBullet;
        }

        if phi.events.now.key_2 == Some(true) {
            self.player.cannon = CannonType::SineBullet {
                amplitude: 10.0,
                angular_vel: 15.0,
            };
        }

        if phi.events.now.key_3 == Some(true) {
            // TODO
        }

        // Render the ship
        phi.renderer.copy_sprite(&self.player.sprites[self.player.current as usize],
                                 self.player.rect);

        //Render the asteroids
        for asteroid in &self.asteroids {
            asteroid.render(phi);
        }

        //Render the bullets
        for bullet in &self.bullets {
            bullet.render(phi);
        }
        // Render the foreground
        self.bg.front.render(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}
