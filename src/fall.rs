pub trait Falls {
    fn compute_accel(
        &mut self,
        left_pop: &[Box<dyn Falls>],
        right_pop: &[Box<dyn Falls>],
        ns_per_frame: u64,
    ) {
        for other in left_pop.iter() {
            self.compute_fn(other.as_ref(), ns_per_frame);
        }

        for other in right_pop.iter() {
            self.compute_fn(other.as_ref(), ns_per_frame);
        }
    }

    fn compute_fn(&mut self, other: &dyn Falls, ns_per_frame: u64) {
        let dx = other.get_x() - self.get_x();
        let dy = other.get_y() - self.get_y();
        let dist = (dx * dx + dy * dy).sqrt();
        let force = BIG_G * self.get_mass() * other.get_mass() / (dist * dist);

        //we have force and direction, use to modify x and y_vel
        let angle = dy.atan2(dx);
        self.set_x_vel(self.get_x_vel() + (force * angle.cos() / self.get_mass()));
        self.set_y_vel(self.get_y_vel() + (force * angle.sin() / self.get_mass()));

        //then use x and y vel to modify x and y with respect to the framerate
        let seconds_per_frame = ns_per_frame as f32 / 1_000_000_000.;
        self.set_x(self.get_x() + (self.get_x_vel() * seconds_per_frame));
        self.set_y(self.get_y() + (self.get_y_vel() * seconds_per_frame));
    }

    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);
    fn get_mass(&self) -> f32;
    fn get_x_vel(&self) -> f32;
    fn get_y_vel(&self) -> f32;
    fn set_x_vel(&mut self, x_vel: f32);
    fn set_y_vel(&mut self, y_vel: f32);
}

pub const LITTLE_G: f32 = 9.81;
pub const BIG_G: f32 = 6.67430e-11;
