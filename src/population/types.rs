use crate::sparse::CsrMatrix;


pub struct Population {
    /// Id for the population
    pub id: usize,
    /// Number of neurons in the population
    pub size: usize,
    /// Intrapopulation connection weight (unsigned, sign is determined via neurons type within the population)
    pub intra_weight: f32,
    /// Sparsity of the intrapopulation connections, between 0 and 1
    pub intra_sparsity: f32,
    /// Proportion of excitatory neurons in the population, between 0 and 1
    pub excitatory_ratio: f32,
}

/// Matrix that holds the configuration between all populations.
/// Each entry at (x, y) is a tuple (sparsity, weight)
/// that tells the sparsity of connections and their associated weights from population x to y.
pub type PopulationsConfiguration = CsrMatrix<(f32, f32)>;