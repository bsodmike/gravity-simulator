use crate::data::Population;
use crate::point_mass::PointMass;
use crate::rerun::write_log;
use crate::time::TimeConfig;
use crate::RenderMode;
use rand::prelude::*;
use rand::Rng;

pub fn run_random_simulation(
    framerate: u64,
    duration_ns: u64,
    num_points: usize,
    max_init_speed: f32,
    max_mass: f32,
    spawn_radius: f32,
    render_mode: RenderMode,
) {
    let ns_per_frame: u64 = 1_000_000_000 / framerate;
    let mut time = TimeConfig::new(ns_per_frame);
    let mut population = Population::new(num_points);
    let mut pop_colours = Vec::with_capacity(num_points);
    let mut radii = Vec::with_capacity(num_points);

    let mut rr = rerun::RecordingStreamBuilder::new("gravity sim")
        .spawn()
        .unwrap();

    let mut rng = rand::rngs::StdRng::from_entropy();

    for _ in 0..num_points {
        population.add(PointMass::new(
            nalgebra::Point3::new(
                rng.gen_range(-spawn_radius..spawn_radius),
                rng.gen_range(-spawn_radius..spawn_radius),
                rng.gen_range(-spawn_radius..spawn_radius),
            ),
            nalgebra::Vector3::new(
                rng.gen_range(-max_init_speed..max_init_speed),
                rng.gen_range(-max_init_speed..max_init_speed),
                rng.gen_range(-max_init_speed..max_init_speed),
            ),
            rng.gen_range(1.0..max_mass),
        ));
        let mass = rand::random::<f32>() * max_mass;
        let radius = 5e-1 * mass.powf(1.0 / 3.0);

        pop_colours.push(rerun::Color::from_u32(rand::random::<u32>()));
        radii.push(radius);
    }

    // NOTE: Add super heavy mass
    population.add(PointMass::new(
        nalgebra::Point3::new(0., 0., 0.),
        nalgebra::Vector3::new(0.0, 0.0, 0.0),
        1e17,
    ));
    pop_colours.push(rerun::Color::from_u32(0));
    radii.push(600.);

    let mut population2 = population.clone();
    rr.set_time_nanos("stable_time", time.get_time() as i64);

    write_log(&mut rr, &mut population, &pop_colours, &radii, &render_mode);

    let mut i = 0;
    while time.get_time() < duration_ns {
        rr.set_time_nanos("stable_time", time.get_time() as i64);

        Population::compute_next_positions(&mut population, &mut population2, ns_per_frame);

        write_log(&mut rr, &mut population, &pop_colours, &radii, &render_mode);

        if i % 100000 == 0 {
            println!("Frame: {}", i);
        }
        i += 1;

        std::mem::swap(&mut population, &mut population2);

        time.advance_frame();
    }
}
