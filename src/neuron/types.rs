/**
 * The model of the neurons is based on Izhikevich model. He suggested that, in practice, all inhibitory neurons are modelized 
 * using Fast Spiking (FS) dynamics while excitatory are modelized using Regular Spiking (RS) dynamics
 * https://www.izhikevich.org/publications/spikes.pdf
*/

use super::constants::{INIT_CONDITION, RS_BASE_PARAMS, FS_BASE_PARAMS};

#[derive(Debug, Copy, Clone)]
pub struct NeuronParams {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct NeuronState {
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum CellType {
    RS,    // Use Regular Spiking kind to represent excitatory populations
    FS,    // Use Fast Spiking kind to represent inhibitory populations
}

#[derive(Debug, Copy, Clone)]
pub struct Neuron {
    pub id: usize,
    pub params: NeuronParams,
    pub state: NeuronState,
}

impl Neuron {
    pub fn new(id: usize, params: NeuronParams) -> Self {
        Self {
            id,
            params,
            state: INIT_CONDITION
        }
    }

    /// Returns the neuron parameters given the cell type of the neuron
    pub fn get_params_for_type(cell_type: CellType) -> NeuronParams {
        let mut rng = rand::thread_rng();
        let r: f32 = rnd.gen() // Random float 0.0 to 1.0

        match cell_type{
            CellType::RS => NeuronParams {
                a: RS_BASE_PARAMS.a,
                b: RS_BASE_PARAMS.b,
                c: RS_BASE_PARAMS.c + (15.0 * r.powi(2)),
                d: RS_BASE_PARAMS.d + (6.0  * r.powi(2)),
            },
            CellType::FS => NeuronParams {
                a: RS_BASE_PARAMS.a + (0.08 * r),
                b: RS_BASE_PARAMS.b - (0.05 * r),
                c: RS_BASE_PARAMS.c,
                d: RS_BASE_PARAMS.d,
            }
        }
    }
}