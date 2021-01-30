use crate::map::*;

pub struct Ray {
    x: f64,
    y: f64,
    angle: f64,
}

impl Ray {
    const MAX_DIST: f64 = 100.0;

    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self { x, y, angle }
    }

    pub fn cast(mut self, map: &Map) -> RayHit {
        let (angle_sin, angle_cos) = self.angle.sin_cos();
        let angle_tan = angle_sin / angle_cos;

        let mut dist = 0.0;

        while dist < Self::MAX_DIST
            && map.get_tile(self.x as usize, self.y as usize) == Tile::Empty
        {
            let x_frac = self.x.fract();
            let y_frac = self.y.fract();

            let x_diff = if angle_cos > 0.0 {
                1.0 - x_frac
            } else if x_frac == 0.0 {
                -1.0
            } else {
                -x_frac
            };
            let vert_dist = (x_diff / angle_cos).abs();

            let y_diff = if angle_sin > 0.0 {
                1.0 - y_frac
            } else if y_frac == 0.0 {
                -1.0
            } else {
                -y_frac
            };
            let horiz_dist = (y_diff / angle_sin).abs();

            if vert_dist < horiz_dist {
                dist += vert_dist;
                self.x += x_diff * 1.001;
                self.y += x_diff * angle_tan;
            } else {
                dist += horiz_dist;
                self.x += y_diff / angle_tan;
                self.y += y_diff * 1.001;
            }
        }

        RayHit {
            dist,
            x: self.x,
            y: self.y,
        }
    }
}

pub struct RayHit {
    pub dist: f64,
    pub x: f64,
    pub y: f64,
}
