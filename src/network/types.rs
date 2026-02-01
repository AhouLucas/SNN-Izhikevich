use rand::Rng;

use crate::neuron::{Neuron, CellType};
use crate::sparse::{CsrMatrix};
use crate::population::{self, Population, PopulationsConfiguration};

/// Structure containing all the neuronal connections
/// and configuration of the whole network
#[derive(Debug)]
pub struct Network {
    /// List of all the neurons of the network
    pub neurons: Vec<Neuron>,
    /// List of populations to describe the configuration of the populations
    pub populations: Vec<Population>,
    /// List of index ranges `(start, end)`, with `end` not included, each population is responsible for
    pub population_ranges: Vec<(usize, usize)>,
    /// Adjacency matrix for the network that contains the weight x -> y at index (x, y)
    pub synapses: CsrMatrix<f32>,
    /// Buffer to accumulate the spikes emitted at the previous time step for each neuron
    pub spikes_buffer: Vec<f32>,
}


impl Network {

    pub fn new(populations: Vec<Population>, populations_config: PopulationsConfiguration) -> Self {

        let mut neurons: Vec<Neuron> = Vec::new();

        // Holds the ranges of neuron's index each population is responsible for (used for matrix building)
        let mut population_ranges: Vec<(usize, usize)> = Vec::with_capacity(populations.len());

        let mut triplets: Vec<(usize, usize, f32)> = Vec::new();

        for pop in populations.iter() {
            let pop_id = pop.id;
            let n_neurons = pop.size;
            let exc_ratio = pop.excitatory_ratio;
            let n_exc: usize = (n_neurons as f32 * exc_ratio) as usize;

            // Offset for the indices of the global matrix
            let offset = neurons.len();
            population_ranges.push((0 + offset, n_neurons + offset));

            // Initialize neuron for this population
            for x in 0..n_neurons {

                // Initialize neuron as excitatory/inhibitory with ratio exc_ratio
                let cell_type: CellType;
                if x < n_exc {
                    cell_type = CellType::RS;
                } else {
                    cell_type = CellType::FS;
                }

                let neuron = Neuron::new_rnd(neurons.len(), pop_id, cell_type);
                neurons.push(neuron);
            }
        }

        // Build connections between populations
        for src_pop_idx in 0..populations.len() {
            for target_pop_idx in 0..populations.len() {
                let config = populations_config.get_or(src_pop_idx, target_pop_idx, &(0., 0.));
                let sparsity = config.0; let weight = config.1;

                let src_idx_range = population_ranges[src_pop_idx];
                let target_idx_range = population_ranges[target_pop_idx];

                for src_neuron_idx in src_idx_range.0..src_idx_range.1 {
                    // Sign of the connection weight (+1 if src neuron is excitatory, -1 otherwise)
                    let sign = if neurons[src_neuron_idx].cell_type.is_excitatory() {1.0} else {-1.0};

                    for target_neuron_idx in target_idx_range.0..target_idx_range.1 {
                        let r: f32 = rand::random();

                        // Add weight with probability `sparsity`
                        if r < sparsity {
                            triplets.push((src_neuron_idx, target_neuron_idx, sign * weight));
                        }
                    }
                }
            }
        }

        let synapses = CsrMatrix::from_triplets(neurons.len(), neurons.len(), &triplets);

        Network {
            neurons,
            populations,
            population_ranges,
            synapses,
            spikes_buffer: Vec::new(),
        }
    }

}