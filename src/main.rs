mod neuron; // This imports src/neuron/mod.rs

use neuron::Neuron; // Import the struct from the module
use std::fs::{self, File}; // Added 'self' to handle directory creation
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 1. Initialize
    // This will use RS_BASE_PARAMS and INIT_CONDITION internally
    let mut neuron = Neuron::new_rs(1);
    
    // 2. Setup Simulation
    let dt = 0.5; 
    let total_steps = 2000; // 1000 ms

    // 3. Ensure data directory exists and create file
    fs::create_dir_all("data")?; 
    let mut file = File::create("data/neuron_output.csv")?;
    writeln!(file, "time_ms,voltage,spike")?;

    println!("Starting simulation...");

    // 4. The Loop
    for i in 0..total_steps {
        let t = i as f32 * dt;
        
        // Input: 10.0 current between 100ms and 900ms
        let input = if t > 100.0 && t < 900.0 { 10.0 } else { 0.0 };
        
        // Step physics
        let spiked = neuron.step(input, dt);
        
        // Write data
        let v_display = if spiked { 30.0 } else { neuron.state.v };
        writeln!(file, "{:.2},{:.4},{}", t, v_display, if spiked {1} else {0})?;
    }

    println!("Done. Results saved to data/neuron_output.csv");
    Ok(())
}