#[derive(PartialEq)]
pub enum Tile {
    Empty,
    Wall,
}

pub struct Map {}

impl Map {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        if x == 0 || y == 0 {
            Tile::Wall
        } else if x & 0b1111 == 0 {
            if (y + 4) & 0b1000 != 0 && (y >> 4).count_ones() & 3 == 1 {
                Tile::Empty
            } else {
                Tile::Wall
            }
        } else if y & 0b1111 == 0 {
            if (x + 4) & 0b1000 != 0 && (x >> 4).count_ones() & 3 == 1 {
                Tile::Empty
            } else {
                Tile::Wall
            }
        } else {
            Tile::Empty
        }
    }
}
