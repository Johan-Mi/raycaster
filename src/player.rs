pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Player {
    pub const MOVE_SPEED: f64 = 20.0;
    pub const TURN_SPEED: f64 = 4.0;

    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
        }
    }
}
