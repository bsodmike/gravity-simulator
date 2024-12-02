use crate::data::Population;
use crate::prelude::NewtonianMechanics;
use crate::RenderMode;
use rerun::RecordingStream;
use rerun::{Vec2D, Vec3D};

pub fn write_log(
    rr: &mut RecordingStream,
    population: &mut Population,
    colours: &Vec<rerun::Color>,
    radii: &Vec<f32>,
    render_mode: &RenderMode,
) {
    match render_mode {
        RenderMode::TwoD => write_log_2d(rr, population, colours, radii),
        RenderMode::ThreeD => write_log_3d(rr, population, colours, radii),
    }
}
pub fn write_log_2d(
    rr: &mut RecordingStream,
    population: &mut Population,
    colours: &Vec<rerun::Color>,
    radii: &Vec<f32>,
) {
    match rr.log(
        "gravity_sim",
        &rerun::Points2D::new(
            population
                .get()
                .iter()
                .map(|p| {
                    let position = p.get_position();
                    Vec2D::new(position.x, position.y)
                })
                .collect::<Vec<Vec2D>>(),
        )
        .with_colors(colours.clone())
        .with_radii(radii.clone()),
    ) {
        Ok(_) => (),
        Err(e) => println!("Error logging frame: {:?}", e),
    };
}

pub fn write_log_3d(
    rr: &mut RecordingStream,
    population: &mut Population,
    colours: &Vec<rerun::Color>,
    radii: &Vec<f32>,
) {
    match rr.log(
        "gravity_sim",
        &rerun::Points3D::new(
            population
                .get()
                .iter()
                .map(|p| {
                    let position = p.get_position();
                    Vec3D::new(position.x, position.y, position.z)
                })
                .collect::<Vec<Vec3D>>(),
        )
        .with_colors(colours.clone())
        .with_radii(radii.clone()),
    ) {
        Ok(_) => (),
        Err(e) => println!("Error logging frame: {:?}", e),
    };
}
