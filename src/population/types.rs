use crate::sparse::CsrMatrix;

#[derive(Debug)]
pub struct Population {
    /// Id for the population
    pub id: usize,
    /// Number of neurons in the population
    pub size: usize,
    /// Proportion of excitatory neurons in the population, between 0 and 1
    pub excitatory_ratio: f32,
}

/// Matrix that holds the configuration between all populations.
/// Each entry at (x, y) is a tuple (sparsity, weight)
/// that tells the sparsity of connections and their associated weights from population x to y.
pub type PopulationsConfiguration = CsrMatrix<(f32, f32)>;