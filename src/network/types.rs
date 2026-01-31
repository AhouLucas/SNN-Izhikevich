use rand::Rng;

use crate::neuron::{Neuron, CellType};
use crate::sparse::{CsrMatrix};
use crate::population::{Population, PopulationsConfiguration};

/// Structure containing all the neuronal connections
/// and configuration of the whole network
pub struct Network {
    /// List of all the neurons of the network
    neurons: Vec<Neuron>,
    /// List of populations to describe the configuration of the populations
    populations: Vec<Population>,
    /// List of index ranges each population is responsible for
    population_ranges: Vec<(usize, usize)>,
    /// Adjacency matrix for the network that contains the weight x -> y at index (x, y)
    synapses: CsrMatrix<f32>,
    /// Buffer to accumulate the spikes emitted at the previous time step for each neuron
    spikes_buffer: Vec<f32>,
}


impl Network {

    pub fn new(populations: Vec<Population>, populations_config: PopulationsConfiguration) {

        let mut neurons: Vec<Neuron> = Vec::new();

        // Holds the ranges of neuron's index each population is responsible for (used for matrix building)
        let mut population_ranges: Vec<(usize, usize)> = Vec::with_capacity(populations.len());

        let mut triplets: Vec<(usize, usize, f32)> = Vec::new();

        for pop in populations.iter() {
            let pop_id = pop.id;
            let n_neurons = pop.size;
            let exc_ratio = pop.excitatory_ratio;
            let n_exc: usize = (n_neurons as f32 * exc_ratio) as usize;
            let sparsity = pop.intra_sparsity;
            let weight = pop.intra_weight;

            // Offset for the indices of the global matrix
            let offset = neurons.len();
            population_ranges.push((0 + offset, n_neurons + offset - 1));

            // Initialize neuron for this population
            // and add connection triplet for intrapopulation connection
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

                // Connect this neuron to other neuron within this population
                // with weight (+/-) `weight` and probability `sparsity`
                let w_sign: f32 = if cell_type.is_excitatory() { 1.0 } else { -1.0 };

                for y in 0..n_neurons {
                    let r: f32 = rand::random();

                    if r < sparsity {
                        triplets.push((x + offset, y + offset, w_sign * weight));
                    }
                }
            }
        }


        // Build connections between populations
        // TODO
    }

}