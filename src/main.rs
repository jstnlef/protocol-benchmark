mod simulator;

use simulator::NetworkSimulator;
use std::error::Error;

fn main() {
    let sim = NetworkSimulator::new(0, 100, 0);
    let sim_thread = sim.start();
    sim_thread.join().unwrap();
}
