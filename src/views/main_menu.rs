use phi::{Phi, View, ViewAction, gfx};
use phi::gfx::{CopySprite, Sprite};
use sdl2::pixels::Color;
use phi::data::Rectangle;

struct Action {
    /// The function which should be executed if the action is chosen.
    func: Box<Fn(&mut Phi) -> ViewAction>,
    idle_sprite: Sprite,
    hover_sprite: Sprite,
}
impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi) -> ViewAction>) -> Action {

        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label,
                                            "assets/belligerent.ttf",
                                            32,
                                            Color::RGB(216, 21, 21))
                .unwrap(),
            hover_sprite: phi.ttf_str_sprite(label,
                                             "assets/belligerent.ttf",
                                             38,
                                             Color::RGB(255, 255, 255))
                .unwrap(),
        }
    }
}
pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        MainMenuView {
            actions: vec![Action::new(phi,
                                      "New Game",
                                      Box::new(|phi| {
                                                   ViewAction::ChangeView(Box::new(::views::game::ShipView::new(phi)))
                                               })),
                          Action::new(phi, "Quit", Box::new(|_| ViewAction::Quit))],
            selected: 0,
        }
    }
}
impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.now.key_space == Some(true) {
            return (self.actions[self.selected as usize].func)(phi);
        }
        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }
        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected > self.actions.len() as i8 - 1 {
                self.selected = 0;
            }
        }
        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();
        let (win_w, win_h) = phi.output_size();


        for (i, action) in self.actions.iter().enumerate() {
            if self.selected as usize == 1 {
                let (w, h) = action.hover_sprite.size();
                phi.renderer.copy_sprite(&action.hover_sprite,
                                         Rectangle {
                                             x: (win_w - w) / 2.0,
                                             //? We place every element under the previous one.
                                             y: 32.0 + 48.0 * i as f64,
                                             w: w,
                                             h: h,
                                         });
            } else {
                let (w, h) = action.idle_sprite.size();
                phi.renderer.copy_sprite(&action.idle_sprite,
                                         Rectangle {
                                             x: (win_w - w) / 2.0,
                                             //? We place every element under the previous one.
                                             y: 32.0 + 48.0 * i as f64,
                                             w: w,
                                             h: h,
                                         });
            }

        }
        ViewAction::None
    }
}
