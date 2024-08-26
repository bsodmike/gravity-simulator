use gravity_sim::sim::run_random_simulation;
use gravity_sim::sim::NUM_ENTITIES;
fn main() {
    run_random_simulation(1000, 30000000000000, NUM_ENTITIES, 0., 1000000000., 1000.);
}
