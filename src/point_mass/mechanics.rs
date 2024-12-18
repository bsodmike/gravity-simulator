use crate::prelude::*;

pub trait NewtonianMechanics<T: SimdComplexField> {
    fn get_mass(&self) -> T;

    fn get_position(&self) -> Point3<T>;
    fn get_velocity(&self) -> Vector3<T>;
    fn set_position(&mut self, position: Point3<T>);
    fn set_velocity(&mut self, direction: Vector3<T>);

    fn compute_force_vec(&self, other: &Self) -> Option<Vector3<T>>;
}
