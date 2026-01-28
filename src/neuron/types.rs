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

impl NeuronParams {

    /// Return randomized parameters for neuron given its cell type
    /// as suggested in Izhikevich paper to have enough heterogeneity
    pub fn get_rnd_neuron_params_per_type(cell_type: CellType) -> Self {
        let r: f32 = rand::random();

        match cell_type {
            CellType::RS => NeuronParams {
                a: RS_BASE_PARAMS.a,
                b: RS_BASE_PARAMS.b,
                c: RS_BASE_PARAMS.c + (15.0 * r.powi(2)),
                d: RS_BASE_PARAMS.d - (6.00 * r.powi(2)),
            },
            CellType::FS => NeuronParams {
                a: FS_BASE_PARAMS.a + (0.08 * r),
                b: FS_BASE_PARAMS.b - (0.05 * r),
                c: FS_BASE_PARAMS.c,
                d: FS_BASE_PARAMS.d,
            },
        }
    }
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

    pub fn new_rnd(id: usize, cell_type: CellType) -> Self {
        let params = NeuronParams::get_rnd_neuron_params_per_type(cell_type);
        Self::new(id, params)
    }

    pub fn step(&mut self, input_current: f32, dt: f32) -> bool {
        let v_prev = self.state.v;
        let u_prev = self.state.u;

        let dv = (0.04 * v_prev.powi(2) + 5.0 * v_prev + 140.0 - u_prev + input_current) * dt;
        let du = (self.params.a * (self.params.b * v_prev - u_prev)) * dt;

        self.state.v += dv;
        self.state.u += du;

        if self.state.v >= 30.0 {
            self.state.v = self.params.c;
            self.state.u += self.params.d;
            return true; // Has spiked
        }

        return false;
    }
}