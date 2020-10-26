use crate::game::*;
use crate::ray::*;
use piston_window::*;

pub const SCREEN_WIDTH: f64 = 640.0;
pub const SCREEN_HEIGHT: f64 = 360.0;

const FOV: f64 = 1.25;

const WALL_HEIGHT_SCALE: f64 = 1.8;

pub fn render(context: &Context, gfx: &mut G2d, game: &Game) {
    clear(color::BLACK, gfx);

    for row in 0..SCREEN_WIDTH as usize {
        let angle_difference =
            (row as f64 / SCREEN_WIDTH * 2.0 - 1.0).atan() * FOV;
        let angle = game.player.angle + angle_difference;
        let hit = Ray::new(game.player.x, game.player.y, angle).cast(&game.map);
        let dist = hit.dist * angle_difference.cos();
        let wall_height = SCREEN_HEIGHT * WALL_HEIGHT_SCALE / dist;
        let shade = (-(hit.dist as f32).sqrt() / 4.0).exp();
        let wall_color = [shade, shade, shade, 1.0];

        let l = [
            row as f64,
            SCREEN_HEIGHT / 2.0 - wall_height,
            row as f64,
            SCREEN_HEIGHT / 2.0 + wall_height,
        ];
        line(wall_color, 0.5, l, context.transform, gfx);
    }
}
