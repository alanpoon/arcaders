use phi::{Phi, View, ViewAction};
use phi::gfx::{CopySprite, Sprite, AnimatedSprite, AnimatedSpriteDescr};
use phi::data::{Rectangle, MaybeAlive};
use sdl2::pixels::Color;
use sdl2::mixer::{Chunk, Music};
use std::path::Path;
use views::bullets::*;
use views::shared::BgSet;

// Constants
const PLAYER_SPEED: f64 = 180.0;
const PLAYER_PATH: &'static str = "assets/spaceship.png";
const PLAYER_W: f64 = 43.0;
const PLAYER_H: f64 = 39.0;
const DEBUG: bool = false;
const ASTEROID_PATH: &'static str = "assets/asteroid.png";
const ASTEROIDS_WIDE: usize = 21;
const ASTEROIDS_HIGH: usize = 7;
const ASTEROIDS_TOTAL: usize = ASTEROIDS_WIDE * ASTEROIDS_HIGH - 4;
const ASTEROID_SIDE: f64 = 96.0;
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
    fn update(mut self, dt: f64) -> Option<Explosion> {
        self.alive_since += dt;
        self.sprite.add_time(dt);

        if self.alive_since >= EXPLOSION_DURATION {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }
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
    fn at_center(&self, center: (f64, f64)) -> Explosion {
        // FPS in [10.0, 30.0)
        let sprite = self.sprite.clone();

        Explosion {
            sprite: sprite,

            // In the screen vertically, and over the right of the screen
            // horizontally.
            rect: Rectangle::with_size(EXPLOSION_SIDE, EXPLOSION_SIDE).center_at(center),

            alive_since: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
enum PlayerFrame {
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
struct Player {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: PlayerFrame,
    cannon: CannonType,
}
impl Player {
    pub fn new(phi: &mut Phi) -> Player {
        // Get the spaceship's sprites
        let spritesheet = Sprite::load(&mut phi.renderer, PLAYER_PATH).unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                                                    w: PLAYER_W,
                                                    h: PLAYER_H,
                                                    x: PLAYER_W * x as f64,
                                                    y: PLAYER_H * y as f64,
                                                })
                                 .unwrap());
            }
        }

        Player {
            // Spawn the player at the center of the screen, vertically.
            rect: Rectangle {
                x: 64.0,
                y: (phi.output_size().1 - PLAYER_H) / 2.0,
                w: PLAYER_W,
                h: PLAYER_H,
            },
            sprites: sprites,
            current: PlayerFrame::MidNorm,
            cannon: CannonType::RectBullet,
        }
    }
    pub fn render(&self, phi: &mut Phi) {
        // Render the bounding box (for debugging purposes)
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.rect.to_sdl().unwrap());
        }
        // Render the Player
        phi.renderer.copy_sprite(&self.sprites[self.current as usize], self.rect);
    }
    fn spawn_bullets(&self) -> Vec<Box<Bullet>> {
        let cannons_x = self.rect.x + 30.0;
        let cannon1_y = self.rect.y + 6.0;
        let cannon2_y = self.rect.y + PLAYER_H - 10.0;
        spawn_bullets(self.cannon, cannons_x, cannon1_y, cannon2_y)

    }
    pub fn update(&mut self, phi: &mut Phi, elapsed: f64) {
        if phi.events.now.key_1 == Some(true) {
            self.cannon = CannonType::RectBullet;
        }

        if phi.events.now.key_2 == Some(true) {
            self.cannon = CannonType::SineBullet {
                amplitude: 10.0,
                angular_vel: 15.0,
            };
        }

        if phi.events.now.key_3 == Some(true) {
            // TODO
        }
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
        self.rect.x += dx;
        self.rect.y += dy;
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };
        self.rect = self.rect.move_inside(movable_region).unwrap();
        self.current = if dx == 0.0 && dy < 0.0 {
            PlayerFrame::UpNorm
        } else if dx > 0.0 && dy < 0.0 {
            PlayerFrame::UpFast
        } else if dx < 0.0 && dy < 0.0 {
            PlayerFrame::UpSlow
        } else if dx == 0.0 && dy == 0.0 {
            PlayerFrame::MidNorm
        } else if dx > 0.0 && dy == 0.0 {
            PlayerFrame::MidFast
        } else if dx < 0.0 && dy == 0.0 {
            PlayerFrame::MidSlow
        } else if dx == 0.0 && dy > 0.0 {
            PlayerFrame::DownNorm
        } else if dx > 0.0 && dy > 0.0 {
            PlayerFrame::DownFast
        } else if dx < 0.0 && dy > 0.0 {
            PlayerFrame::DownSlow
        } else {
            unreachable!()
        };
    }
}
pub struct GameView {
    player: Player,
    bullets: Vec<Box<Bullet>>,
    asteroids: Vec<Asteroid>,
    bg: BgSet,
    asteroid_factory: AsteroidFactory,
    explosions: Vec<Explosion>,
    explosion_factory: ExplosionFactory,
    music: Music<'static>,
    bullet_sound: Chunk,
    explosion_sound: Chunk,
}
impl GameView {
    pub fn with_backgrounds(phi: &mut Phi, bg: BgSet) -> GameView {
        let music = Music::from_file(Path::new("assets/mdk_phoenix_orchestral.ogg")).unwrap();
        music.play(-1).unwrap();
        let bullet_sound = Chunk::from_file(Path::new("assets/bullet.ogg")).unwrap();

        let explosion_sound = Chunk::from_file(Path::new("assets/explosion.ogg")).unwrap();
        GameView {
            player: Player::new(phi),
            bullets: vec![],
            asteroids: vec![],
            bg: bg,
            asteroid_factory: Asteroid::factory(phi),
            explosions: vec![],
            explosion_factory: Explosion::factory(phi),
            music: music,
            bullet_sound: bullet_sound,
            explosion_sound: explosion_sound,
        }
    }
}
impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.key_escape == Some(true) {
            let bg = self.bg.clone();
            return ViewAction::ChangeView(Box::new(::views::main_menu::MainMenuView::with_backgrounds(phi,bg)));
        }


        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();
        // Render the Backgrounds
        self.bg.back.render(&mut phi.renderer, elapsed);
        self.bg.middle.render(&mut phi.renderer, elapsed);


        // Update the player
        self.player.update(phi, elapsed);
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
        // Update the explosions
        self.explosions = ::std::mem::replace(&mut self.explosions, vec![])
            .into_iter()
            .filter_map(|explosion| explosion.update(elapsed))
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

                // The player's Player is destroyed if it is hit by an asteroid.
                // In which case, the asteroid is also destroyed.
                if asteroid.rect().overlaps(self.player.rect) {
                    asteroid_alive = false;
                    player_alive = false;
                }

                //? Then, we use the magic of `filter_map` to keep only the asteroids
                //? that didn't explode.
                if asteroid_alive {
                    Some(asteroid)
                } else {
                    self.explosions.push(self.explosion_factory.at_center(asteroid.rect()
                                                                              .center()));
                    None
                }
            })
            .collect();

        self.bullets = transition_bullets.into_iter().filter_map(MaybeAlive::as_option).collect();
        if !player_alive {
            println!("The player's Player has been destroyed.");
        }
        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }
        if ::rand::random::<usize>() % 100 == 0 {
            self.asteroids.push(self.asteroid_factory.random(phi));
        }
        println!("{}", self.asteroids.len());

        //Render the player
        self.player.render(phi);
        //Render the asteroids
        for asteroid in &self.asteroids {
            asteroid.render(phi);
        }

        //Render the bullets
        for bullet in &self.bullets {
            bullet.render(phi);
        }
        //Render Explosion
        for explosion in &self.explosions {
            explosion.render(phi);
        }
        // Render the foreground
        self.bg.front.render(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}
