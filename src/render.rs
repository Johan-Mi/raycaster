use crate::game::*;
use crate::ray::*;
use crate::textures::*;
use piston_window::*;

pub const SCREEN_WIDTH: f64 = 640.0;
pub const SCREEN_HEIGHT: f64 = 360.0;

const FOV: f64 = 1.25;

const WALL_HEIGHT_SCALE: f64 = 2.5;

pub fn render<F, C>(
    context: &Context,
    gfx: &mut G2d,
    texture_context: &mut TextureContext<F, gfx_device_gl::Resources, C>,
    game: &Game,
) where
    C: gfx::CommandBuffer<gfx_device_gl::Resources>,
    F: gfx::Factory<gfx_device_gl::Resources>,
{
    clear(color::BLACK, gfx);

    let mut img =
        im::ImageBuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    for row in 0..SCREEN_WIDTH as usize {
        let angle_difference =
            (row as f64 / SCREEN_WIDTH).mul_add(2.0, -1.0).atan() * FOV;
        let angle = game.player.angle + angle_difference;
        let hit = Ray::new(game.player.x, game.player.y, angle).cast(&game.map);
        let dist = hit.dist * angle_difference.cos();
        let wall_height = (SCREEN_HEIGHT * WALL_HEIGHT_SCALE / dist) as u32 * 2;
        let shade = ((hit.dist as f32).sqrt() / -6.0).exp();
        let wall_height_invisible =
            wall_height.saturating_sub(SCREEN_HEIGHT as u32) / 2;

        let tex_x =
            ((hit.x + hit.y).fract() * WALL_TEXTURE_WIDTH as f64) as usize;

        for i in wall_height_invisible..(wall_height - wall_height_invisible) {
            let tex_y = (WALL_TEXTURE_HEIGHT as f32 * i as f32
                / wall_height as f32
                * 10.0) as usize
                % WALL_TEXTURE_HEIGHT;

            let shade = shade
                * (1.0
                    - (i as f32 / wall_height as f32 - 0.5)
                        * (i as f32 / wall_height as f32 - 0.5));

            let wall_color_unshaded =
                WALL_TEXTURE[tex_y * WALL_TEXTURE_WIDTH + tex_x];
            let wall_color = im::Rgba([
                (wall_color_unshaded[0] as f32 * shade) as u8,
                (wall_color_unshaded[1] as f32 * shade) as u8,
                (wall_color_unshaded[2] as f32 * shade) as u8,
                255,
            ]);

            img.put_pixel(
                row as u32,
                SCREEN_HEIGHT as u32 / 2 + i - wall_height / 2,
                wall_color,
            );
        }

        if wall_height_invisible == 0 {
            for i in 0..(SCREEN_HEIGHT as u32).saturating_sub(wall_height) / 2 {
                let pix_dist =
                    wall_height as f64 / (SCREEN_HEIGHT - 2.0 * i as f64);
                let pix_dist_adjusted = hit.dist * (pix_dist - 1.0);

                let world_x = hit.x + angle.cos() * pix_dist_adjusted;
                let world_y = hit.y + angle.sin() * pix_dist_adjusted;

                let tex_y = (CEILING_TEXTURE_HEIGHT as f64 * (world_y / 8.0))
                    as usize
                    % CEILING_TEXTURE_HEIGHT;
                let tex_x = (CEILING_TEXTURE_WIDTH as f64 * (world_x / 8.0))
                    as usize
                    % CEILING_TEXTURE_WIDTH;

                let ceil_color_unshaded =
                    CEILING_TEXTURE[tex_y * CEILING_TEXTURE_WIDTH + tex_x];
                let shade = ((pix_dist * dist).sqrt() / -6.0).exp();
                let ceil_color = im::Rgba([
                    (ceil_color_unshaded[0] as f64 * shade) as u8,
                    (ceil_color_unshaded[1] as f64 * shade) as u8,
                    (ceil_color_unshaded[2] as f64 * shade) as u8,
                    255,
                ]);
                img.put_pixel(row as u32, i, ceil_color);
                img.put_pixel(
                    row as u32,
                    SCREEN_HEIGHT as u32 - i - 1,
                    ceil_color,
                );
            }
        }
    }

    let tex =
        Texture::from_image(texture_context, &img, &TextureSettings::new())
            .unwrap();

    image(&tex, context.transform, gfx);
}
