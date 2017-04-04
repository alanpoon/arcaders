//define sprite type using Arc
use std::sync::Arc;
use phi::data::Rectangle;
use sdl2::render::{Texture, TextureQuery, Renderer};
use sdl2::image::LoadTexture;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;


//tells compiler to automatically implement the Clone trait
#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}
impl Sprite {
    pub fn clone(&self) -> Sprite {
        Sprite {
            tex: self.tex.clone(),
            src: self.src.clone(),
        }
    }
    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();
        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                w: tex_query.width as f64,
                h: tex_query.height as f64,
                x: 0.0,
                y: 0.0,
            },
        }
    }
    /// Returns a new `Sprite` representing a sub-region of the current one.
    /// The provided `rect` is relative to the currently held region.
    /// Returns `Some` if the `rect` is valid, i.e. included in the current
    /// region, and `None` otherwise.
    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: rect.x + self.src.x,
            y: rect.y + self.src.y,
            ..rect
        };
        if self.src.contains(new_src) {
            Some(Sprite {
                     tex: self.tex.clone(),
                     src: new_src,
                 })
        } else {
            None
        }
    }
    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }
}

impl Renderable for Sprite {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl());
    }
}
#[derive(Clone)]
pub struct AnimatedSprite {
    sprites: Rc<Vec<Sprite>>,
    frame_delay: f64,
    current_time: f64,
}
impl AnimatedSprite {
    pub fn new(sprites: Vec<Sprite>, frame_delay: f64) -> AnimatedSprite {
        AnimatedSprite {
            sprites: Rc::new(sprites),
            frame_delay: frame_delay,
            current_time: 0.0,
        }
    }
    pub fn with_fps(sprites: Vec<Sprite>, fps: f64) -> AnimatedSprite {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::with_fps");
        }
        AnimatedSprite::new(sprites, 1.0 / fps)
    }
    pub fn frames(&self) -> usize {
        self.sprites.len()
    }
    pub fn set_frame_delay(&mut self, frame_delay: f64) {
        self.frame_delay = frame_delay;
    }
    pub fn set_fps(&mut self, fps: f64) {
        if fps == 0.0 {
            panic!("Passed 0 to AnimatedSprite::set_fps");
        }
        self.set_frame_delay(1.0 / fps);
    }
    pub fn add_time(&mut self, dt: f64) {
        self.current_time += dt;
        if self.current_time < 0.0 {
            self.current_time = (self.frames() - 1) as f64 * self.frame_delay;
        }
    }
}
impl Renderable for AnimatedSprite {
    /// Renders the current frame of the sprite.
    fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        let current_frame = (self.current_time / self.frame_delay) as usize % self.frames();

        let sprite = &self.sprites[current_frame];
        sprite.render(renderer, dest);
    }
}
pub trait Renderable {
    fn render(&self, renderer: &mut Renderer, dest: Rectangle);
}
pub trait CopySprite<T> {
    fn copy_sprite(&mut self, sprite: &T, dest: Rectangle);
}
impl<'window, T: Renderable> CopySprite<T> for Renderer<'window> {
    fn copy_sprite(&mut self, renderable: &T, dest: Rectangle) {
        renderable.render(self, dest);
    }
}
