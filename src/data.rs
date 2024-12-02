use crate::point_mass::PointMass;
use crate::prelude::*;

pub type PointMassCollection = Vec<PointMass<f32>>;

#[derive(Clone)]
pub struct Population {
    items: PointMassCollection,
    colour: Vec<rerun::Color>,
    radii: Vec<f32>,
}

impl Population {
    pub fn new(points: usize) -> Self {
        Self {
            items: Vec::with_capacity(points),
            colour: vec![],
            radii: vec![],
        }
    }

    pub fn add(&mut self, point_mass: PointMass<f32>) {
        self.items.push(point_mass);
    }

    pub fn add_colour(&mut self, colour: rerun::Color) {
        self.colour.push(colour);
    }

    pub fn get_colour_collection(&self) -> &Vec<rerun::Color> {
        &self.colour
    }

    pub fn get_radii_collection(&self) -> &Vec<f32> {
        &self.radii
    }

    pub fn add_radius(&mut self, radius: f32) {
        self.radii.push(radius);
    }

    pub fn get(&self) -> &PointMassCollection {
        &self.items
    }

    pub fn get_mut(&mut self) -> &mut PointMassCollection {
        &mut self.items
    }

    pub fn compute_next_positions(pop1: &mut Population, pop2: &mut Population, ns_per_frame: u64) {
        let mut p1_items = pop1.get_mut();
        let p2_items = pop2.get_mut();

        compute_next_positions(&mut p1_items, &p2_items, ns_per_frame);
    }
}

pub fn compute_next_positions<T>(
    next_set: &mut [impl NewtonianMechanics<T>],
    last_set: &[impl NewtonianMechanics<T>],
    ns_per_frame: u64,
) where
    T: SimdComplexField,
{
    let time_const = ns_per_frame as f64 / 1e9;

    for (i, point) in last_set.iter().enumerate() {
        let mut force = Vector3::new(T::zero(), T::zero(), T::zero());

        for other in last_set.iter() {
            if let Some(f) = point.compute_force_vec(other) {
                force += f;
            } else {
                continue;
            }
        }
        let acceleration: Vector3<T> = force / point.get_mass();
        let old_velocity: Vector3<T> = point.get_velocity();

        let new_velocity = old_velocity.clone()
            + (acceleration * T::from_simd_real(nalgebra::convert(time_const)));

        let new_position = point.get_position()
            + (old_velocity + new_velocity.clone())
                * T::from_simd_real(nalgebra::convert(time_const * 0.5));

        next_set[i].set_velocity(new_velocity);
        next_set[i].set_position(new_position);
    }
}
