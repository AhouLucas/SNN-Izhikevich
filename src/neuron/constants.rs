use super::types::{NeuronState, NeuronParams};

pub const INIT_CONDITION: NeuronState = NeuronState {v: -65.0, u: -13.0};

pub const RS_BASE_PARAMS: NeuronParams = NeuronParams {a: 0.02, b: 0.20, c: -65.0, d: 8.0};
pub const FS_BASE_PARAMS: NeuronParams = NeuronParams {a: 0.02, b: 0.25, c: -65.0, d: 2.0};
