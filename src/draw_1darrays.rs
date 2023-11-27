//! Draw 1darray
use crate::prelude::*;
use plotters::prelude::*;

pub fn draw_1darrays(arrays: &[(Array1<f64>, RGBColor)], file_path: &str, title: &str) {
    let root = BitMapBackend::new(file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut max = 1.;
    let mut max_length = 0;
    for arr in arrays {
        max = std::cmp::max_by(max, arr.0.norm_max(), |a, b| a.partial_cmp(b).unwrap());
        max_length = std::cmp::max(max_length, arr.0.len());
    }

    let mut chart = ChartBuilder::on(&root)
        .caption(
            title,
        ("sans-serif", 50).into_font()
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0..max_length,
            -1.5 * max..1.5 * max,
        )
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    for arr in arrays {
        let point_series = PointSeries::<_, _, Circle<_, _>, _>::new(
            arr.0.iter().enumerate().map(|(i, x)| (i, *x)),
            4,
            &arr.1,
        );
        chart.draw_series(point_series).unwrap();
    }
}