use crate::*;

#[repr(align(32))]
pub struct PointMass {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub radius: f32,
    pub x_vel: f32, //ms-1
    pub y_vel: f32, //ms-1
}

impl Falls for PointMass {
    //output here should be a net acceleration vector, and modify the object's velocity and position.

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_x_vel(&self) -> f32 {
        self.x_vel
    }

    fn get_y_vel(&self) -> f32 {
        self.y_vel
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn set_x_vel(&mut self, x_vel: f32) {
        self.x_vel = x_vel;
    }

    fn set_y_vel(&mut self, y_vel: f32) {
        self.y_vel = y_vel;
    }
}
