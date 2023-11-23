//! # Normal OTR TGI method
use super::OTRTGI;
use crate::prelude::*;

pub struct OTRTGINormal {}

impl OTRTGINormal {
    pub fn new() -> Self {
        Self {}
    }
}

impl OTRTGI for OTRTGINormal {
    fn solve_(
        &self,
        reference: &Array2<f64>,
        otr_value: &Array1<f64>,
        step_func: &Array1<f64>,
    ) -> Result<Array1<f64>, Box<dyn Error>> {
        let sample_size = reference.shape()[0];
        let otr_point = reference.shape()[1] -1;
        let otr_average = otr_value.iter().map(|v| *v).sum::<f64>() / sample_size as f64;
        let mut delta_otr_values = otr_value.clone();
        delta_otr_values
            .iter_mut()
            .for_each(|v| *v = *v - otr_average);
        let mut delta_references = reference.clone();
        for j in 0..delta_references.shape()[1] {
            let column = delta_references.slice_mut(s![.., j]);
            let average = column.sum() / sample_size as f64;
            for x in column {
                *x -= average;
            }
        }

        Ok(ArrayBase::from_shape_fn(otr_point + 1, |t| {
            let mut cov = 0.;
            for i in 0..sample_size {
                cov += delta_otr_values[i] * delta_references[[i, t]];
            }
            cov /= sample_size as f64;

            return cov / step_func[otr_point - t];
        }))
    }
}

#[test]
fn otr_tgi_normal_test() {
    pub use plotters::prelude::*;

    std::env::set_var("RUST_BACKTRACE", "1");

    let references = array![
        [1., 1., 1., 1.],
        [1., -1., 1., -1.],
        [1., 1., -1., -1.],
        [1., -1., -1., 1.],
    ];
    let mask = array![0., 2., 1., 0.];
    let step_func: Array1<f64> = ArrayBase::from_shape_fn(
        4,
        |t| 2.0f64.powf(-(t as f64)),
    );
    let mut scaled_ref = references.clone();
    for i in 0..scaled_ref.shape()[1] {
        let column = scaled_ref.slice_mut(s![.., i]);
        for x in column {
            *x *= step_func[3 - i];
        }
    }
    let otr_values = scaled_ref.dot(&mask);

    let otr_tgi = OTRTGINormal::new();
    let result = otr_tgi.solve(&references, &otr_values, &step_func).unwrap();
    assert!((&mask - &result).norm_l2() < 1e-8);

    crate::draw_1darrays::draw_1darrays(
        &[(mask, GREEN), (result, RED)],
        "results/normal_tgi_test.png",
        "normal_tgi_test.png"
    );
}