mod neuron;
mod network;

use network::Population;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let n = 100;
    let mut pop = Population::new(n); // 100 Neurons
    let dt = 0.1;
    let steps = 1000;

    let mut file = File::create("data/raster.csv")?;
    writeln!(file, "n,steps,dt")?;
    writeln!(file, "{},{},{}", n, steps, dt)?;
    writeln!(file, "time,neuron_id")?; // Header for Raster Plot

    for i in 0..steps {
        let t = i as f32 * dt;
        
        // Step the population
        let spiked_ids = pop.step(dt);

        for id in spiked_ids {
            writeln!(file, "{},{}", t, id)?;
        }
    }
    Ok(())
}