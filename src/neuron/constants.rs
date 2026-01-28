use super::types::{NeuronState, NeuronParams};

pub const INIT_CONDITION: NeuronState = NeuronState {v: -65.0, u: -13.0};

pub const RS_BASE_PARAMS: NeuronParams = NeuronParams {0.02, 0.20, -65.0, 8.0};
pub const FS_BASE_PARAMS: NeuronParams = NeuronParams {0.02, 0.25, -65.0, 2.0};
