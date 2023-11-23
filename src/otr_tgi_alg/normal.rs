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

    //描画先をBackendとして指定。ここでは画像に出力するためBitMapBackend
    let root = BitMapBackend::new("results/normal_tgi_test.png", (640, 480)).into_drawing_area();
    //背景を白に
    root.fill(&WHITE).unwrap();

    let max = mask.norm_max();

    //グラフの軸設定など
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "normal_tgi_test.png",
            ("sans-serif", 50).into_font(),
        )
        .margin(10) //上下左右の余白
        .x_label_area_size(30) //x軸ラベル部分の余白
        .y_label_area_size(30) //y軸ラベル部分の余白
        .build_cartesian_2d(
            0..mask.shape()[0], //x軸の設定
            -1.5 * max..1.5 * max,    //y軸の設定
        )
        .unwrap();

    //x軸y軸、グリッド線など描画
    chart.configure_mesh().draw().unwrap();
    //データの描画。(x, y)のイテレータとしてデータ点を渡す。
    let point_series = PointSeries::<_, _, Circle<_, _>, _>::new(
        mask.iter().enumerate().map(|(i, x)| (i, *x)),
        4,
        &RED,
    );
    chart.draw_series(point_series).unwrap();
    let point_series = PointSeries::<_, _, Circle<_, _>, _>::new(
        result.iter().enumerate().map(|(i, x)| (i, *x)),
        4,
        &BLUE,
    );
    chart.draw_series(point_series).unwrap();
}