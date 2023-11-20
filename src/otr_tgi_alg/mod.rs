//! # ORT TGI Alg
//! 
//! Here, otr tgi algorithms are defiend.

mod normal;
mod cs;

pub trait OTRTGI {
    fn solve<T: Iterator<Item = f64>, S: Iterator<Item = T>>(&self, reference: S, otr_value: T, otr_point: usize, step_func: T);
}