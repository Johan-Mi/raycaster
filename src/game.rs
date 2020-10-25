use crate::player::*;
use piston_window::{Key, UpdateArgs};
use std::collections::HashSet;

pub struct Game {
    pub player: Player,
    pub keys: HashSet<Key>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.keys.contains(&Key::D) {
            self.player.x += Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::A) {
            self.player.x -= Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::S) {
            self.player.y += Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::W) {
            self.player.y -= Player::MOVE_SPEED * args.dt;
        }
    }
}
