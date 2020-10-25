pub struct Player {
    pub x: f64,
    pub y: f64,
}

impl Player {
    pub const MOVE_SPEED: f64 = 40.0;

    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
