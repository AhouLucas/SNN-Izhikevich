/// Compressed Sparse Row (CSR) matrix format
/// Efficient for row-wise access and iteration
#[derive(Debug, Clone)]
pub struct CsrMatrix<T> {
    /// Column indices for each non-zero entry
    pub col_indices: Vec<usize>,
    /// Values of non-zero entries
    pub values: Vec<T>,
    /// Row pointers: row_ptr[i] is the index in col_indices/values where row i starts
    /// row_ptr[nrows] = nnz (total number of non-zero entries)
    pub row_ptr: Vec<usize>,
    pub nrows: usize,
    pub ncols: usize,
}

impl<T: Clone> CsrMatrix<T> {
    /// Create an empty CSR matrix with given dimensions
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            col_indices: Vec::new(),
            values: Vec::new(),
            row_ptr: vec![0; nrows + 1],
            nrows,
            ncols,
        }
    }

    /// Build a CSR matrix from COO-style triplets (row, col, value)
    /// Entries do not need to be sorted
    pub fn from_triplets(nrows: usize, ncols: usize, triplets: &[(usize, usize, T)]) -> Self
    where
        T: Default,
    {
        // Count entries per row
        let mut row_counts = vec![0usize; nrows];
        for (r, _, _) in triplets {
            row_counts[*r] += 1;
        }

        // Build row_ptr from counts
        let mut row_ptr = Vec::with_capacity(nrows + 1);
        row_ptr.push(0);
        for count in &row_counts {
            row_ptr.push(row_ptr.last().unwrap() + count);
        }

        // Allocate storage
        let nnz = triplets.len();
        let mut col_indices = vec![0usize; nnz];
        let mut values: Vec<T> = (0..nnz).map(|_| T::default()).collect();

        // Fill in entries (use row_counts as write cursors)
        let mut write_pos = row_ptr[..nrows].to_vec();
        for (r, c, v) in triplets {
            let pos = write_pos[*r];
            col_indices[pos] = *c;
            values[pos] = v.clone();
            write_pos[*r] += 1;
        }

        Self { col_indices, values, row_ptr, nrows, ncols }
    }

    /// Create an empty CSR matrix builder with given number of columns
    /// Use `push_row` to add rows incrementally
    pub fn builder(ncols: usize) -> Self {
        Self {
            col_indices: Vec::new(),
            values: Vec::new(),
            row_ptr: vec![0],
            nrows: 0,
            ncols,
        }
    }

    /// Add a single row to the matrix
    /// Takes an iterator of (column, value) pairs for this row
    pub fn push_row(&mut self, entries: impl IntoIterator<Item = (usize, T)>) {
        for (col, val) in entries {
            self.col_indices.push(col);
            self.values.push(val);
        }
        self.row_ptr.push(self.col_indices.len());
        self.nrows += 1;
    }

    /// Number of non-zero entries
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    /// Get all (column, value) pairs for a given row - O(1) access
    pub fn row_entries(&self, row: usize) -> impl Iterator<Item = (usize, &T)> + '_ {
        let start = self.row_ptr[row];
        let end = self.row_ptr[row + 1];
        self.col_indices[start..end]
            .iter()
            .zip(&self.values[start..end])
            .map(|(&c, v)| (c, v))
    }

    /// Iterate over all entries as (row, col, &value)
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
        (0..self.nrows).flat_map(move |row| {
            self.row_entries(row).map(move |(col, val)| (row, col, val))
        })
    }

    /// Get a single element at (row, col)
    /// Returns None if the element is zero (not stored)
    /// Time complexity: O(k) where k is the number of non-zeros in the row
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.nrows || col >= self.ncols {
            return None;
        }

        let start = self.row_ptr[row];
        let end = self.row_ptr[row + 1];

        // Linear search through the row's column indices
        for i in start..end {
            if self.col_indices[i] == col {
                return Some(&self.values[i]);
            }
        }

        None // Element is zero (not stored)
    }

    /// Get a single element, returning a default value if not found
    pub fn get_or<'a>(&'a self, row: usize, col: usize, default: &'a T) -> &'a T {
        self.get(row, col).unwrap_or(default)
    }
}

impl<'a, T> IntoIterator for &'a CsrMatrix<T>
where
    T: Clone,
{
    type Item = (usize, usize, &'a T);
    type IntoIter = Box<dyn Iterator<Item = (usize, usize, &'a T)> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}
