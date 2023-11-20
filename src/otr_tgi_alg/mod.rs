//! # ORT TGI Alg
//!
//! Here, otr tgi algorithms are defiend.

mod cs;
mod normal;

use crate::prelude::*;

/// this trait provides otr tgi functionality.
pub trait OTRTGI {
    fn solve(
        &self,
        reference: &Array2<f64>,
        otr_value: &Array1<f64>,
        step_func: &Array1<f64>,
    ) -> Result<Array1<f64>> {
        match is_valid_data(reference, otr_value, step_func) {
            Ok(_) => (),
            Err(msg) => {
                return Err(msg);
            }
        }

        self.solve_(reference, otr_value, step_func)
    }
    fn solve_(
        &self,
        reference: &Array2<f64>,
        otr_value: &Array1<f64>,
        step_func: &Array1<f64>,
    ) -> Result<Array1<f64>>;
}

/// check whether data is valid for otr-tgi.
pub fn is_valid_data(
    reference: &Array2<f64>,
    otr_value: &Array1<f64>,
    step_func: &Array1<f64>,
) -> Result<()> {
    if reference.shape()[1] != step_func.shape()[0] || reference.shape()[0] != otr_value.shape()[0]
    {
        return Err(anyhow!(format!(
            "the data size is invalid. reference: {}x{}, otr_value: {}, step_func: {}",
            reference.shape()[0],
            reference.shape()[1],
            otr_value.shape()[0],
            step_func.shape()[0],
        )));
    }

    if step_func.iter().find(|v| **v == 0.).is_some() {
        return Err(anyhow!("step func includes zero."));
    }
    Ok(())
}

#[test]
fn is_valid_data_test() {
    let refe = array![
        [1., 2., 0.5],
        [0.5, 3., 1.5],
    ];
    let otr_v = array![1.2, 3.];
    let step_func = array![1., 0.5, 0.3];

    assert!(crate::otr_tgi_alg::is_valid_data(&refe, &otr_v, &step_func).is_ok());
}