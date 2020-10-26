use crate::map::*;
use crate::player::*;
use piston_window::{Key, UpdateArgs};
use std::collections::HashSet;
use std::f64;

pub struct Game {
    pub player: Player,
    pub keys: HashSet<Key>,
    pub map: Map,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            keys: HashSet::new(),
            map: Map::new(),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.keys.contains(&Key::D) {
            self.player.x -=
                self.player.angle.sin() * Player::MOVE_SPEED * args.dt;
            self.player.y +=
                self.player.angle.cos() * Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::A) {
            self.player.x +=
                self.player.angle.sin() * Player::MOVE_SPEED * args.dt;
            self.player.y -=
                self.player.angle.cos() * Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::S) {
            self.player.x -=
                self.player.angle.cos() * Player::MOVE_SPEED * args.dt;
            self.player.y -=
                self.player.angle.sin() * Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::W) {
            self.player.x +=
                self.player.angle.cos() * Player::MOVE_SPEED * args.dt;
            self.player.y +=
                self.player.angle.sin() * Player::MOVE_SPEED * args.dt;
        }
        if self.keys.contains(&Key::E) {
            self.player.angle += Player::TURN_SPEED * args.dt;
            self.player.angle %= f64::consts::TAU;
        }
        if self.keys.contains(&Key::Q) {
            self.player.angle +=
                f64::consts::TAU - Player::TURN_SPEED * args.dt;
            self.player.angle %= f64::consts::TAU;
        }
    }
}
