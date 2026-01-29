mod neuron;
mod population;
mod sparse;

use population::Population;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 1. Create a small population
    let mut pop = Population::new(20, 0.9, 30., 0.5);

    // 2. Wire them up (Daisy Chain)
    // 0 -> 1 -> 2 -> 3
    // let strong_weight = 200.0; // Needs to be strong enough to cause a spike
    // pop.connect(0, 1, strong_weight);
    // pop.connect(1, 2, strong_weight);
    // pop.connect(2, 3, strong_weight);

    let mut file = File::create("data/chain_test.csv")?;
    writeln!(file, "time,neuron_id")?;

    let dt = 0.5;
    
    // 3. Run Simulation
    for i in 0..1000 { // 500 ms
        let t = i as f32 * dt;
        
        // STIMULATE ONLY NEURON 0
        // We need to modify step() to accept a vector of currents, 
        // OR just rely on the fact that I_ext is uniform for now.
        // Hack for testing: Let's assume step() takes uniform I_ext, 
        // but we want to target only #0.
        
        // CHALLENGE: Your current step() applies I_ext to EVERYONE.
        // To test this properly, you should set I_ext = 0.0, 
        // and artificially inject current into Neuron 0 manually before the step.
        
        // Let's modify the buffer directly for Neuron 0 to jumpstart it
        if i == 10 { // At t=5ms, kick Neuron 0
             pop.spikes_buffer[0] += 50.0; 
        }
        
        // Run with 0.0 background current
        let spikes = pop.step(0.0, dt);

        for id in spikes {
            writeln!(file, "{},{}", t, id)?;
        }
    }
    Ok(())
}