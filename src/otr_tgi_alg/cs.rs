//! # Algorithm for OTRTGI with CS
use super::OTRTGI;
use crate::prelude::*;

/// calculate recovered data thas is sparse on sparse_basis'columns.
pub struct OTRTGICS {
    sparse_alg: Box<dyn sparse_modeling::sparse_alg::SparseAlg>,
    sparse_basis: Array2<f64>,
}

impl OTRTGICS {
    pub fn new(sparse_alg: Box<dyn sparse_modeling::sparse_alg::SparseAlg>, sparse_basis: &Array2<f64>,) -> Self {
        Self{ sparse_alg, sparse_basis: sparse_basis.clone() }
    }
}

impl OTRTGI for OTRTGICS {
    fn solve_(
            &self,
            reference: &Array2<f64>,
            otr_value: &Array1<f64>,
            step_func: &Array1<f64>,
    ) -> Result<Array1<f64>, Box<dyn Error>> {
        let otr_point = reference.shape()[1] - 1;
        let mut matrix = reference.clone();
        for j in 0..matrix.shape()[1] {
            let column = matrix.slice_mut(s![.., j]);
            for x in column {
                *x *= step_func[otr_point - j];
            }
        }
        let matrix = matrix.dot(&self.sparse_basis);

        let sparse_rep = self.sparse_alg.solve(&matrix, otr_value)?;

        Ok(self.sparse_basis.dot(&sparse_rep))
    }
}