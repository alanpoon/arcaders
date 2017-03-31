//define sprite type using Arc
use std::sync::Arc;
use phi::data::Rectangle;
use sdl2::render::{Texture, TextureQuery,Renderer};
use sdl2::image::LoadTexture;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;   

//tells compiler to automatically implement the Clone trait 
#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}
impl Sprite{
    pub fn clone(&self)->Sprite{
        Sprite{
            tex:self.tex.clone(),
            src: self.src.clone()
        }
    }
    pub fn load(renderer:&Renderer,path:&str){
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new);
    }
    pub fn new(texture:Texture)->Sprite{
        let tex_query = texture.query();
        Sprite{
            tex:Rc::new(RefCell::new(texture)),
            src:Rectangle{
                w:tex_query.width as f64,
                h: tex_query.height as f64,
                x:0.0,
                y:0.0
            }
        }
    }
     /// Returns a new `Sprite` representing a sub-region of the current one.
    /// The provided `rect` is relative to the currently held region.
    /// Returns `Some` if the `rect` is valid, i.e. included in the current
    /// region, and `None` otherwise.
    pub fn region(&self,rect:Rectangle)->Option<Sprite>{
        let new_src = Rectangle{
            x:rect.x+self.src.x,
            y:rect.y+self.src.y,
            ..rect
        };
        if self.src.contains(new_src){
            Some(Sprite{tex:self.tex.clone(),src:new_src})
        } else {
            None
        }
    }
        pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }
    pub fn render(&self, renderer: &mut Renderer, dest: Rectangle){
        renderer.copy(&mut self.tex.borrow_mut(),self.src.to_sdl(),dest.to_sdl());
    }
}