#[macro_use]
mod events;
pub mod data;
pub mod gfx;
use self::gfx::Sprite;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::path::Path;
use sdl2::ttf::Sdl2TtfContext;
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left:Left,
        key_right:Right,
        key_space: Space,
        key_enter: Return,
        key_1:Num1,
        key_2:Num2,
        key_3:Num3
    },
    else: {
        quit: Quit { .. }
    }
}
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
    pub ttf_context: &'window Sdl2TtfContext,
    cached_fonts: HashMap<(&'static str, i32), ::sdl2::ttf::Font<'window, 'window>>,
}
impl<'window> Phi<'window> {
    fn new(events: Events,
           renderer: Renderer<'window>,
           ttf_context: &'window Sdl2TtfContext)
           -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
            ttf_context: ttf_context,
            cached_fonts: HashMap::new(),
        }
    }
    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }
    pub fn ttf_str_sprite(&mut self,
                          text: &str,
                          font_path: &'static str,
                          size: i32,
                          color: Color)
                          -> Option<Sprite> {
        if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
            return font.render(text)
                       .blended(color)
                       .ok()
                       .and_then(|surface| {
                                     self.renderer.create_texture_from_surface(&surface).ok()
                                 })
                       .map(Sprite::new);
        }
        self.ttf_context
            .load_font(Path::new(font_path), size as u16)
            .ok()
            .and_then(|font| {
                          self.cached_fonts.insert((font_path, size), font);
                          self.ttf_str_sprite(text, font_path, size, color)
                      })

    }
}


pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

//2nd argument takes closure, Box<View> to relieve the defaultview
pub fn spawn<F>(title: &str, init: F)
    where F: Fn(&mut Phi) -> Box<View>
{
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _image_context = ::sdl2::image::init(::sdl2::image::INIT_PNG).unwrap();
    let _ttf_context = ::sdl2::ttf::init().unwrap();
    let _mixer_context = ::sdl2::mixer::init(::sdl2::mixer::INIT_OGG).unwrap();
    ::sdl2::mixer::open_audio(44100, ::sdl2::mixer::AUDIO_S16LSB, 2, 1024).unwrap();
    ::sdl2::mixer::allocate_channels(32);
    // Create the window
    let window = video.window(title, 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();
    // Create Phi Context
    let mut context = Phi::new(Events::new(sdl_context.event_pump().unwrap()),
                               window.renderer()
                                   .accelerated()
                                   .build()
                                   .unwrap(),
                               &_ttf_context);

    //create default view using a box
    let mut current_view: Box<View> = init(&mut context);

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    // Prepare the events record

    loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }
        before = now;
        fps += 1;
        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        context.events.pump(&mut context.renderer);
        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view,
        }
    }

}
