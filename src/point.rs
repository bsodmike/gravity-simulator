use crate::*;

#[derive(Copy, Clone)]
#[repr(align(32))]
pub struct PointMass {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub radius: f32,
    pub x_vel: f32, //ms-1
    pub y_vel: f32, //ms-1
}

impl PointMass {
    pub fn compute_accel(
        &mut self,
        left_pop: &[PointMass],
        right_pop: &[PointMass],
        ns_per_frame: u64,
    ) {
        let start_vel_x = self.x_vel;
        let start_vel_y = self.y_vel;

        let mut result_force_x = 0.;
        let mut result_force_y = 0.;

        let time_constant = ns_per_frame as f32 / 1e9;

        for other in left_pop.iter() {
            let vals = self.compute_force_vec(other);
            result_force_x -= vals.x;
            result_force_y -= vals.y;
        }

        for other in right_pop.iter() {
            let vals = self.compute_force_vec(other);
            result_force_x -= vals.x;
            result_force_y -= vals.y;
        }

        let new_speed_x = start_vel_x + (result_force_x / self.mass) * time_constant;
        let new_speed_y = start_vel_y + (result_force_y / self.mass) * time_constant;

        self.x_vel = new_speed_x;
        self.y_vel = new_speed_y;
        self.x = self.get_x() + (start_vel_x + new_speed_x) / 2. * time_constant;
        self.y = self.get_y() + (start_vel_y + new_speed_y) / 2. * time_constant;
    }

    pub fn compute_force_vec(&self, other: &PointMass) -> Vec2D {
        let dx = other.get_x() - self.get_x();
        let dy = other.get_y() - self.get_y();
        let dist = (dx * dx + dy * dy).sqrt();
        let force = 10000. * -self.get_mass() * other.get_mass() / (dist * dist);

        //we have force and direction, use to modify x and y_vel
        let angle = dy.atan2(dx);

        let force_x = force * angle.cos();
        let force_y = force * angle.sin();

        if self.get_radius() + other.get_radius() < dist {
            //no collision
            return Vec2D::new(force_x, force_y);
        };

        //collision
        let k = Vec2D::new(dx / dist, dy / dist);

        let initial_vel = Vec2D::new(self.get_x_vel(), self.get_y_vel());
        let other_vel = Vec2D::new(other.get_x_vel(), other.get_y_vel());

        let a = 2. * k * (initial_vel - other_vel) / (1. / self.get_mass() + 1. / other.get_mass());

        let delta_mv = a * k;

        initial_vel - delta_mv / self.get_mass()
    }

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

    fn get_radius(&self) -> f32 {
        self.radius
    }
}

unsafe impl Send for PointMass {}
unsafe impl Sync for PointMass {}
