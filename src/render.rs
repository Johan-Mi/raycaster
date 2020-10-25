use crate::game::*;
use piston_window::*;

pub const SCREEN_WIDTH: f64 = 640.0;
pub const SCREEN_HEIGHT: f64 = 360.0;

pub fn render(context: &Context, gfx: &mut G2d, game: &Game) {
    clear(color::BLACK, gfx);

    let rect = rectangle::square(game.player.x, game.player.y, 8.0);
    Rectangle::new(color::WHITE).draw(rect, &context.draw_state, context.transform, gfx);
}
