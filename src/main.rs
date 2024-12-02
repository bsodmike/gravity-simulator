use clap::Parser;
use gravity_simulator::RenderMode;
use gravity_simulator::{cli, simulate::run_random_simulation};

const NUM_ENTITIES: usize = 100;
const FRAME_RATE: u64 = 60000;
const DURATION_NS: u64 = 100_000_000_000;
const MAX_INIT_SPEED: f32 = 1000.0;
const MAX_MASS: f32 = 100000000.0;
const SPAWN_RADIUS: f32 = 10000.0;

fn main() {
    let mut mode = RenderMode::default();
    let cli = cli::Args::parse();

    if cli.render == String::from("2D") {
        mode = RenderMode::TwoD
    }

    run_random_simulation(
        FRAME_RATE,
        DURATION_NS,
        NUM_ENTITIES,
        MAX_INIT_SPEED,
        MAX_MASS,
        SPAWN_RADIUS,
        mode,
    );
}
