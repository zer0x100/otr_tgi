use ndarray_linalg::SVD;

use super::OTRTGI;
use crate::prelude::*;

pub struct OTRTGICS {
    sparse_alg: Box<dyn sparse_modeling::sparse_alg::SparseAlg>,
    sparse_map: Array2<f64>,
    inv_sparse_map: Array2<f64>,
}

impl OTRTGICS {
    pub fn new(sparse_alg: Box<dyn sparse_modeling::sparse_alg::SparseAlg>, sparse_map: &Array2<f64>,) -> Result<Self, Box<dyn Error>> {
        if sparse_map.shape()[0] != sparse_map.shape()[1] {
            return Err(From::from("sparse_map needs to be reversible.it is not a square matrix."));
        }
        let inv_map = sparse_modeling::math_func::pseudo_inverse(&sparse_map)?;
        let mut identity: Array2<f64> = ArrayBase::zeros((sparse_map.shape()[0], sparse_map.shape()[1]));
        for i in 0..sparse_map.shape()[0] {
            identity[[i, i]] = 1.0;
        }
        if (&inv_map - &inv_map).norm_max() > F64_EPS {
            return Err(From::from("sparse_map needs to be reversible."));
        }
        Ok(Self { sparse_alg, sparse_map: sparse_map.clone(), inv_sparse_map: inv_map})
    }
}

impl OTRTGI for OTRTGICS {
    fn solve_(
            &self,
            reference: &Array2<f64>,
            otr_value: &Array1<f64>,
            step_func: &Array1<f64>,
        ) -> Result<Array1<f64>, Box<dyn Error>> {
        let mut matrix = reference.clone();
        self.sparse_alg.solve(&matrix, otr_value)?;
    }
}