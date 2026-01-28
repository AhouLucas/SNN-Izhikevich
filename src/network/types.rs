use crate::neuron::{Neuron, CellType};

pub struct Population {
    pub neurons: Vec<Neuron>,
    /// Adjacency matrix for population. Index `i` contains a list of `(target_neuron_id, weight)`
    pub synapses: Vec<Vec<(usize, f32)>>, 
    /// Buffer to accumulate the synaptic current for the next step
    pub spikes_buffer: Vec<f32>, 
}

impl Population {
    /// Create a new population of size `size` with a ratio of excitatory neurons of `excitatory` 
    pub fn new(size: usize, excitatory_ratio: f32) -> Self {
        
        // Neurons initialization
        if excitatory_ratio < 0. || excitatory_ratio > 1. {
            panic!("Wrong Argument: expected excitatory_ratio to be between 0 and 1, got {}", excitatory_ratio);
        }

        let mut neurons = Vec::new();
        let n_exc = (size as f32 * excitatory_ratio) as usize;

        for i in 0..size {
            if i < n_exc {
                neurons.push(Neuron::new_rnd(i, CellType::RS));
            } else {
                neurons.push(Neuron::new_rnd(i, CellType::FS));
            }
        }

        // Create empty adjacency matrix
        let mut synapses = vec![Vec::new(); size];
        let spikes_buffer = vec![0.0; size];

        Population {
            neurons,
            synapses,
            spikes_buffer
        }
    }
    
    /// Connect neuron `from_id` to `to_id` with a connection with weight `weight`
    pub fn connect(&mut self, from_id: usize, to_id: usize, weight: f32) {
        self.synapses[from_id].push((to_id, weight));
    }


    /// Perform a single step with step size `dt` of the Izhikevich model for each neuron in the population
    /// that is subject to an external current `ext_current`.
    pub fn step(&mut self, ext_current: f32, dt: f32) -> Vec<usize> {
        // Tracks which neuron has emitted a spike at the current time step
        let mut spiked_ids = Vec::new();

        // Temporary buffer to accumulate spikes for next time step
        let mut temp_spikes_buffer = vec![0.0; self.neurons.len()];

        // Buffer 
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            // Compute total input
            let i_syn = self.spikes_buffer[i];
            let i_total = ext_current + i_syn;

            // Reset buffer
            self.spikes_buffer[i] = 0.0;

            let did_spike = neuron.step(i_total, dt);

            // Propagate the spikes to every neuron it is connected to for the next time step
            if did_spike {
                spiked_ids.push(neuron.id);

                for &(target_id, weight) in &self.synapses[i] {
                    temp_spikes_buffer[target_id] += weight;
                }
            }
        }

        // Swap buffers for next time step
        self.spikes_buffer =  temp_spikes_buffer;

        spiked_ids
    }
}