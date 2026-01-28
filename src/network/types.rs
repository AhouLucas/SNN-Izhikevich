use crate::neuron::{Neuron, CellType};

pub struct Population {
    pub neurons: Vec<Neuron>,
}

impl Population {
    pub fn new(size: usize) -> Self {
        let mut neurons = Vec::new();

        for i in 0..size {
            neurons.push(Neuron::new_rnd(i, CellType::RS)); // All excitatory for now, need to change later. TODO
        }

        Population {
            neurons,
        }
    }

    pub fn step(&mut self, dt: f32) -> Vec<usize> {
        // Tracks which neuron has emitted a spike at the current time step
        let mut spiked_ids = Vec::new();

        for neuron in &mut self.neurons {
            let has_spiked = neuron.step(20.0, dt); // Constant input current because no connections for now. TODO
            if has_spiked {
                spiked_ids.push(neuron.id);
            }
        }

        spiked_ids
    }
}