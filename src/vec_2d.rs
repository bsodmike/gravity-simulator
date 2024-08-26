use std::ops::{Div, Mul, MulAssign, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Vec2D {
        Vec2D { x, y }
    }
}

impl Mul<f32> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f32) -> Vec2D {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2D> for f32 {
    type Output = Vec2D;

    fn mul(self, rhs: Vec2D) -> Vec2D {
        Vec2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl MulAssign<f32> for Vec2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Vec2D> for Vec2D {
    type Output = f32;

    fn mul(self, rhs: Vec2D) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div<f32> for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: f32) -> Vec2D {
        Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
