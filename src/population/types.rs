use rand::Rng;
use std::collections::HashSet;

use crate::neuron::{Neuron, CellType};
use crate::sparse::{CooMatrix};

pub struct Population {
    /// Number of neurons in the population
    pub size: usize,
    /// List of neurons
    pub neurons: Vec<Neuron>,
    /// Adjacency matrix for population. Index `i` contains a list of `(target_neuron_id, weight)`
    pub synapses: CooMatrix, 
    /// Buffer to accumulate the synaptic current for the next step
    pub spikes_buffer: Vec<f32>,
}

impl Population {
    /// Create a new population of size `size` with a ratio of excitatory neurons of `excitatory` 
    pub fn new(size: usize, excitatory_ratio: f32, interneuronal_weight: f32, sparsity: f32) -> Self {
        
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

        // Create sparse random adjacency matrix with negative/positive weights for inhibitory/excitatory neurons
        let expected_nnz = (size as f32 * sparsity).ceil() as usize;
        let mut synapses = CooMatrix::with_capacity(size, size, expected_nnz);

        let mut rng = rand::thread_rng();

        for src in 0..size {
            for target in 0..size {
                let r: f32 = rng.r#gen();
                // Negative weight if source neuron is inhibitory, positive if excitatory
                let sign = if neurons[src].cell_type.is_excitatory() { 1 } else { -1 } as f32;
                if r < sparsity {
                    synapses.insert(src, target, sign * interneuronal_weight);
                }
            }
        }

        let spikes_buffer = vec![0.0; size];

        Population {
            size,
            neurons,
            synapses,
            spikes_buffer,
        }
    }
    
    /// Connect neuron `from_id` to `to_id` with a connection with weight amplitude `weight_strength`
    /// whose signs depends on whether neuron `from_id` is inhibitory/excitatory
    pub fn connect(&mut self, from_id: usize, to_id: usize, weight_strength: f32) {
        let sign = if self.neurons[from_id].cell_type.is_excitatory() { 1 } else { -1 } as f32;
        self.synapses.insert(from_id, to_id, sign * weight_strength);
    }


    /// Perform a single step with step size `dt` of the Izhikevich model for each neuron in the population
    /// that is subject to an external current `ext_current`.
    pub fn step(&mut self, ext_current: f32, dt: f32) -> HashSet<usize> {
        // Tracks which neuron has emitted a spike at the current time step
        let mut has_spiked = HashSet::with_capacity(self.size);

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

            // Register spikes
            if did_spike {
                has_spiked.insert(neuron.id);
            }
        }

        // Propagate spikes between neurons for next time step
        for (src_id, target_id, weight) in &self.synapses {
            // If src has emitted a spike, add a current `weight` to target
            if has_spiked.contains(&src_id) {
                temp_spikes_buffer[target_id] += weight;
            }
        }

        // Swap buffers for next time step
        self.spikes_buffer =  temp_spikes_buffer;

        has_spiked
    }
}