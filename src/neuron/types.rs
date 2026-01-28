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

    pub fn new_rs(id: usize) -> Self {
        Self::new(id, RS_BASE_PARAMS)
    }

    pub fn new_fs(id: usize) -> Self {
        Self::new(id, FS_BASE_PARAMS)
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