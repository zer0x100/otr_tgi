//! # ORT TGI Alg
//! 
//! Here, otr tgi algorithms are defiend.

mod normal;
mod cs;

use crate::prelude::*;

/// this trait provides otr tgi functionality.
pub trait OTRTGI {
    fn solve(&self, reference: &Array2<f64>, otr_value: &Array1<f64>, step_func: &Array1<f64>) -> Result<Array1<f64>> {
        match is_valid_data(reference, otr_value, step_func) {
            Ok(_) => (),
            Err(msg) => { return Err(msg); },
        }

        self.solve_(reference, otr_value, step_func)
    }
    fn solve_(&self, reference: &Array2<f64>, otr_value: &Array1<f64>, step_func: &Array1<f64>) -> Result<Array1<f64>>;
}

fn is_valid_data(reference: &Array2<f64>, otr_value: &Array1<f64>, step_func: &Array1<f64>) -> Result<()> {
    if reference.shape()[1] != step_func.shape()[0]
        || reference.shape()[0] != otr_value.shape()[0]
    {
        return Err(anyhow!(format!("the data size is invalid. reference: {}x{}, otr_value: {}, step_func: {}",
            reference.shape()[0], reference.shape()[1], otr_value.shape()[0], step_func.shape()[0],
        )))
    }

    if step_func.iter().find(|v| **v == 0.).is_some() {
        return Err(anyhow!("step func includes zero."));
    }
    Ok(())
}