mod game;
mod map;
mod player;
mod ray;
mod render;
mod textures;
extern crate gfx;
extern crate gfx_device_gl;
extern crate image as im;
use game::*;
use piston_window::*;
use render::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Raycaster", (SCREEN_WIDTH, SCREEN_HEIGHT))
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let mut game = Game::new();

    game.player.x = 2005.0;
    game.player.y = 1000.0;

    while let Some(event) = window.next() {
        if let Some(button) = event.press_args() {
            if let Button::Keyboard(key) = button {
                game.keys.insert(key);
            }
        }

        if let Some(button) = event.release_args() {
            if let Button::Keyboard(key) = button {
                game.keys.remove(&key);
            }
        }

        if let Some(args) = event.update_args() {
            game.update(&args);
        }

        window.draw_2d(&event, |context, gfx, _| {
            render(&context, gfx, &mut texture_context, &game)
        });
    }
}
