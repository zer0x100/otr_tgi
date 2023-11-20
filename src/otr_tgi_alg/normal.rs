use super::OTRTGI;
use crate::prelude::*;

pub struct OTRTGINormal {}

impl OTRTGI for OTRTGINormal {
    fn solve(&self, reference: Array2<f64>, otr_value: Array1<f64>, otr_point: usize, step_func: Array1<f64>) -> Array1<f64> {
        
    }
}