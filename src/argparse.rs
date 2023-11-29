//! # Arg Parse
//! define a structure for command line argument parse.
//! the structure provides how-to-use message of 'otr-tgi'

use clap::{ Parser, ValueEnum };

///OTR TGI
///
/// 'otr-tgi' calucalute otr-tgi(one time read time domain ghost imaging) result.
#[derive(Parser)]
pub struct Args {
    /// The number of test samples.
    #[arg(long)]
    pub sample_size: usize,
    /// Otr point.
    #[arg(long)]
    pub otr_point: usize,
    /// Reference signals file(csv)
    ///
    /// 2d csv file required.(i, j) is a value of j-th time in i-th sample.
    #[arg(long)]
    pub references: String,
    /// Otr values file.
    ///
    /// 1d csv file required.
    /// (0, i) is a value of the otr point in the i-th sample.
    #[arg(long)]
    pub otr_values: String,
    ///Slow Step Function File
    ///
    /// 1d csv file. (0, i) is the value of i-th time in slow step function used for the otr tgi.
    #[arg(long)]
    pub step_func: String,
    ///file name to save recovered mask data in. its extension need to be .csv.
    #[arg(long)]
    pub fname: String,
    /// directory to save mask file.
    #[arg(long)]
    pub dir: String,
    /// CS method.
    /// 
    /// if you want to use cs, set a method.[available: FOCUSS, OMP, ISTA, FISTA]
    #[arg(long)]
    cs_method: Option<CSMethod>,
    /// CS method's iterator size
    #[arg(long)]
    iter_num: Option<usize>,
    /// CS Sparse Basis's file name
    #[arg(long)]
    pub sparse_basis: Option<String>,
    /// Lasso's lambda value (Lasso: minimize -> lambda * ||x||_1 + 1/2 * ||y - Ax||_2 ^{2})
    #[arg(long)]
    lasso_lambda: Option<f64>,
}

impl Args {
    pub fn get_cs_method(&self) -> Option<Box<dyn sparse_modeling::sparse_alg::SparseAlg>> {
        if let Some(method) = self.cs_method {
            match method {
                CSMethod::OMP => Some(Box::new(sparse_modeling::sparse_alg::Omp::new(0.))),
                CSMethod::FOCUSS => Some(Box::new(sparse_modeling::sparse_alg::L1Focuss::new(0., self.iter_num.unwrap(), true))),
                CSMethod::ISTA => Some(Box::new(sparse_modeling::sparse_alg::SparseAlgLasso::new(
                    self.lasso_lambda.unwrap(),
                    Box::new(sparse_modeling::lasso_alg::LassoIsta::new(self.iter_num.unwrap(), 0.)),
                    true))),
                CSMethod::FISTA => Some(Box::new(sparse_modeling::sparse_alg::SparseAlgLasso::new(
                    self.lasso_lambda.unwrap(),
                    Box::new(sparse_modeling::lasso_alg::LassoFista::new(self.iter_num.unwrap(), 0.)),
                    true))),
            }
        } else {
            None
        }

    }
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum CSMethod {
    OMP,
    FOCUSS,
    ISTA,
    FISTA,
}