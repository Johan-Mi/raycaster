mod game;
mod player;
mod render;
use game::*;
use piston_window::*;
use render::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Raycaster", (SCREEN_WIDTH, SCREEN_HEIGHT))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut game = Game::new();

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

        window.draw_2d(&event, |context, gfx, _| render(&context, gfx, &game));
    }
}
