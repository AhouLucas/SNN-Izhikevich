use rand::Rng;

/// Coordinate (COO) sparse matrix format
/// Stores non-zero entries as (row, col, value) triplets
#[derive(Debug, Clone)]
pub struct CooMatrix {
    pub entries: Vec<(usize, usize, f32)>,
    pub nrows: usize,
    pub ncols: usize,
}

impl CooMatrix {
    /// Create an empty COO matrix with given dimensions
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            entries: Vec::new(),
            nrows,
            ncols,
        }
    }

    /// Create with pre-allocated capacity
    pub fn with_capacity(nrows: usize, ncols: usize, capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            nrows,
            ncols,
        }
    }

    /// Create a random sparse matrix with uniform weight
    pub fn random(nrows: usize, ncols: usize, sparsity: f32, weight: f32) -> Self {
        let mut rng = rand::thread_rng();
        let expected_nnz = ((nrows * ncols) as f32 * sparsity).ceil() as usize;

        let mut entries = Vec::with_capacity(expected_nnz);

        for r in 0..nrows {
            for c in 0..ncols {
                if rng.r#gen::<f32>() < sparsity {
                    entries.push((r, c, weight));
                }
            }
        }

        Self { entries, nrows, ncols }
    }

    /// Insert a single entry
    pub fn insert(&mut self, row: usize, col: usize, value: f32) {
        debug_assert!(row < self.nrows && col < self.ncols);
        self.entries.push((row, col, value));
    }

    /// Number of non-zero entries
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }

    /// Iterate over entries as (row, col, value)
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, f32)> + '_ {
        self.entries.iter().copied()
    }

    /// Get all targets and weights for a given source row
    pub fn row_entries(&self, row: usize) -> impl Iterator<Item = (usize, f32)> + '_ {
        self.entries
            .iter()
            .filter(move |(r, _, _)| *r == row)
            .map(|(_, c, v)| (*c, *v))
    }
}

impl<'a> IntoIterator for &'a CooMatrix {
    type Item = (usize, usize, f32);
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, (usize, usize, f32)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter().copied()
    }
}

