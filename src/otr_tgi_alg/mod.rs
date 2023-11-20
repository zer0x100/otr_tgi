//! # ORT TGI Alg
//! 
//! Here, otr tgi algorithms are defiend.

mod normal;
mod cs;

use crate::prelude::*;

pub trait OTRTGI {
    fn solve(&self, reference: Array2<f64>, otr_value: Array1<f64>, otr_point: usize, step_func: Array1<f64>) -> Array1<f64>;
}