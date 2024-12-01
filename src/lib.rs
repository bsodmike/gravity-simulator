pub mod point_mass;
pub mod simulate;
pub mod time;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::point_mass::mechanics::NewtonianMechanics;
    pub use nalgebra::{distance_squared, Point3, SimdComplexField, Vector3};
}
