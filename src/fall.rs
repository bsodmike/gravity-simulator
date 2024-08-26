// pub trait Falls {
//     fn compute_accel(
//         &mut self,
//         left_pop: &[Box<dyn Falls>],
//         right_pop: &[Box<dyn Falls>],
//         ns_per_frame: u64,
//     ) {
//         let start_vel_x = self.get_x_vel();
//         let start_vel_y = self.get_y_vel();
//         let mut result_force_x = 0.;
//         let mut result_force_y = 0.;

//         let time_constant = ns_per_frame as f32 / 1e9;

//         for other in left_pop.iter() {
//             let vals = self.compute_force_vec(other.as_ref());
//             result_force_x -= vals[0];
//             result_force_y -= vals[1];
//         }

//         for other in right_pop.iter() {
//             let vals = self.compute_force_vec(other.as_ref());
//             result_force_x -= vals[0];
//             result_force_y -= vals[1];
//         }

//         let new_speed_x = start_vel_x + (result_force_x / self.get_mass()) * time_constant;
//         let new_speed_y = start_vel_y + (result_force_y / self.get_mass()) * time_constant;

//         self.set_x_vel(new_speed_x);
//         self.set_y_vel(new_speed_y);

//         self.set_x(self.get_x() + (start_vel_x + new_speed_x) / 2. * time_constant);
//         self.set_y(self.get_y() + (start_vel_y + new_speed_y) / 2. * time_constant);
//     }

//     fn compute_force_vec(&mut self, other: &dyn Falls) -> [f32; 2] {
//         let dx = other.get_x() - self.get_x();
//         let dy = other.get_y() - self.get_y();
//         let dist = (dx * dx + dy * dy).sqrt();
//         let force = -self.get_mass() * other.get_mass() / (dist * dist);

//         //we have force and direction, use to modify x and y_vel
//         let angle = dy.atan2(dx);

//         [force * angle.cos(), force * angle.sin()]
//     }

//     fn get_x(&self) -> f32;
//     fn get_y(&self) -> f32;
//     fn set_x(&mut self, x: f32);
//     fn set_y(&mut self, y: f32);
//     fn get_mass(&self) -> f32;
//     fn get_x_vel(&self) -> f32;
//     fn get_y_vel(&self) -> f32;
//     fn set_x_vel(&mut self, x_vel: f32);
//     fn set_y_vel(&mut self, y_vel: f32);
// }

// pub const LITTLE_G: f32 = 9.81;
pub const BIG_G: f32 = 6.67430e-11;
