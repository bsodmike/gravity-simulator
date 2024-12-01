use crate::prelude::*;
use mechanics::NewtonianMechanics;

pub mod mechanics;

#[derive(Clone, Debug)]
pub struct PointMass<T: SimdComplexField> {
    position: Point3<T>,
    velocity: Vector3<T>,
    mass: T,
}

impl<T: SimdComplexField> PointMass<T> {
    pub fn new(position: Point3<T>, velocity: Vector3<T>, mass: T) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }
}

const G: f32 = 6.67430e-5;

macro_rules! impl_mechanics_for_pointmass {
    ($t:ty) => {
        impl NewtonianMechanics<$t> for PointMass<$t> {
            fn get_mass(&self) -> $t {
                self.mass
            }

            fn get_position(&self) -> Point3<$t> {
                self.position
            }
            fn get_velocity(&self) -> Vector3<$t> {
                self.velocity
            }
            fn set_position(&mut self, position: Point3<$t>) {
                self.position = position;
            }
            fn set_velocity(&mut self, velocity: Vector3<$t>) {
                self.velocity = velocity;
            }

            fn compute_force_vec(&self, other: &Self) -> Option<Vector3<$t>> {
                if self.get_position() == other.get_position() {
                    return None;
                }
                let dist2 = distance_squared(&self.get_position(), &other.get_position());

                let force = <$t>::from(G) * self.get_mass() * other.get_mass() / dist2;

                let direction = other.get_position() - self.get_position();

                Some((direction.normalize() * (force)).into())
            }
        }
    };
}

impl_mechanics_for_pointmass!(f32);
impl_mechanics_for_pointmass!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_force_vec() {
        let point1 = PointMass::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 1e4);
        let point2 = PointMass::new(Point3::new(2.0, 3.0, 6.0), Vector3::new(0.0, 0.0, 0.0), 1e4);

        let force = point1.compute_force_vec(&point2).unwrap();
        assert!(
            force
                - Vector3::new(0.2857142857142857, 0.42857142857142855, 0.8571428571428571) * 1e8
                    / 49.0
                < Vector3::new(1e-6, 1e-6, 1e-6)
        );
    }
}
