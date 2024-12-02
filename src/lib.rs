pub mod cli;
pub mod data;
pub mod point_mass;
pub mod rerun;
pub mod simulate;
pub mod time;

#[derive(Debug, Default)]
pub enum RenderMode {
    // Render in 2D
    TwoD,

    // Render in 3D
    #[default]
    ThreeD,
}

#[allow(unused_imports)]
pub mod prelude {
    pub use super::point_mass::mechanics::NewtonianMechanics;
    pub use nalgebra::{distance_squared, Point3, SimdComplexField, Vector3};
}
