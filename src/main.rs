use crate::{network::types::Network, population::{Population, PopulationsConfiguration}, sparse::CsrMatrix};

mod neuron;
mod population;
mod sparse;
mod network;

fn main() {
    let populations = vec![
        Population {
            id: 0,
            size: 2,
            excitatory_ratio: 1.0,
        },
        Population {
            id: 1,
            size: 2,
            excitatory_ratio: 0.0,
        }
    ];

    let mut populations_config: PopulationsConfiguration = CsrMatrix::builder(2);
    populations_config.push_row(vec![(0, (1., 10.)), (1, (0.3, 5.))]);
    populations_config.push_row(vec![(0, (0.3, 5.)), (1, (1., 10.))]);
    let network = Network::new(populations, populations_config);

    let synapses = network.synapses;

    for item in &synapses {
        println!("{:?}", item);
    }
}