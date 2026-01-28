pub mod neuron;

use neuron::{Neuron, NeuronParams};

fn main() {
    let n: Neuron = Neuron::new(1, NeuronParams{a:1.0, b:1.0, c:1.0, d:1.0});
    println!("{:?}", n);
}
