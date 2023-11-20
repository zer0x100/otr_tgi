//! # Normal OTR TGI method
use super::OTRTGI;
use crate::prelude::*;

pub struct OTRTGINormal {}

impl OTRTGI for OTRTGINormal {
    fn solve_(
        &self,
        reference: &Array2<f64>,
        otr_value: &Array1<f64>,
        step_func: &Array1<f64>,
    ) -> Result<Array1<f64>> {
        let sample_size = reference.shape()[0];
        let otr_point = reference.shape()[1];
        let otr_average = otr_value.iter().map(|v| *v).sum::<f64>() / sample_size as f64;
        let mut delta_otr_values = otr_value.clone();
        delta_otr_values
            .iter_mut()
            .for_each(|v| *v = *v - otr_average);
        let ref_average = reference.columns()
            .into_iter()
            .map(|column| column.sum() / sample_size as f64);
        let mut delta_references: Array2<f64> = ArrayBase::zeros((sample_size, 0));
        reference
            .columns()
            .into_iter()
            .zip(ref_average)
            .map(|(column, average)| {
                column.to_owned() / average
            })
            .for_each(|column| {
                delta_references.push_column(column.view()).expect("can't add a column");
            }
        );

        Ok(ArrayBase::from_shape_fn( otr_point, |t| {
            let mut cov = 0.;
            for i in 0..sample_size {
                cov += delta_otr_values[i] * delta_references[[i, t]];
            }
            cov /= sample_size as f64;

            return cov / step_func[otr_point - t];
        }))
    }
}
