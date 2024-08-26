use rayon::prelude::*;
use rerun::Vec2D;

use crate::*;

pub fn run_random_simulation(
    framerate: u64,
    duration_ns: u64,
    num_points: usize,
    max_init_speed: f32,
    max_mass: f32,
    spawn_radius: f32,
) {
    let ns_per_frame: u64 = 1_000_000_000 / framerate;
    let mut time = TimeConfig::new(ns_per_frame);
    let mut population: Vec<PointMass> = Vec::with_capacity(num_points);
    let mut pop_colours = Vec::with_capacity(num_points);
    let mut radii = Vec::with_capacity(num_points);

    let rr = rerun::RecordingStreamBuilder::new("gravity sim")
        .spawn()
        .unwrap();

    for _ in 0..num_points {
        let x = rand::random::<f32>() * spawn_radius;
        let y = rand::random::<f32>() * spawn_radius;
        let mass = rand::random::<f32>() * max_mass;
        let radius = mass.powf(1.0 / 3.0);
        let x_vel = rand::random::<f32>() * max_init_speed;
        let y_vel = rand::random::<f32>() * max_init_speed;

        population.push(PointMass {
            x,
            y,
            mass,
            radius,
            x_vel,
            y_vel,
        });

        pop_colours.push(rerun::Color::from_u32(rand::random::<u32>()));
        radii.push(radius);
    }

    let mut i = 0;
    while time.get_time() < duration_ns {
        rr.set_time_nanos("stable_time", time.get_time() as i64);

        population = population
            .par_iter()
            .map(|p| p.compute_accel(&population, ns_per_frame))
            .collect();

        match rr.log(
            "gravity_sim",
            &rerun::Points2D::new(
                population
                    .iter()
                    .map(|p| Vec2D::new(p.x, p.y))
                    .collect::<Vec<Vec2D>>(),
            )
            .with_colors(pop_colours.clone())
            .with_radii(radii.clone()),
        ) {
            Ok(_) => (),
            Err(e) => println!("Error logging frame: {:?}", e),
        }
        if i % 1000 == 0 {
            println!("Frame: {}", i);
        }
        i += 1;
        time.advance_frame();
    }
}
