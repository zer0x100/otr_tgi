//! # OTR TGI
//!
//! 'otr-tgi' is for otr-tgi(one time read time domain ghost imaging).
//! <otr-tgi> --help command shows short description and how to use of this program.
mod argparse;
mod csv_converter;
mod otr_tgi_alg;
mod draw_1darrays;

mod prelude {
    pub use ndarray::prelude::*;
    pub use ndarray_linalg::Norm;
    pub use std::error::Error;
}

use crate::prelude::*;
use clap::Parser;
use plotters::style::RED;

fn main() -> Result<(), Box<dyn Error>> {
    //parse command line arguments.
    let args = argparse::Args::parse();

    //convert csv data to String
    let references = csv_converter::csv_to_2darray(
        &std::fs::read_to_string(&args.references)?,
        (args.sample_size, args.otr_point+1),
    )?;
    let otr_values = csv_converter::csv_to_1darray(
        &std::fs::read_to_string(&args.otr_values)?,
        args.sample_size,
    )?;

    let step_func =
        csv_converter::csv_to_1darray(&std::fs::read_to_string(&args.step_func)?, args.otr_point+1)?;

    let otr_tgi: Box<dyn otr_tgi_alg::OTRTGI>;
    if let Some(cs_method) = args.get_cs_method() {
        let sparse_basis = if let Some(basis) = args.sparse_basis {
            csv_converter::csv_to_2darray(
                &std::fs::read_to_string(basis)?,
                (args.otr_point+1, args.otr_point+1),
            )?
        } else {
            ArrayBase::from_shape_fn(
                (args.otr_point+1, args.otr_point+1),
                |(i, j)| {
                    if i == j {
                        1.0
                    } else {
                        0.
                    }
                })
        };

        otr_tgi = Box::new(otr_tgi_alg::cs::OTRTGICS::new(
            cs_method,
                &sparse_basis,
            )
        );
    } else {
        otr_tgi = Box::new(otr_tgi_alg::normal::OTRTGINormal::new())
    };

    let otr_tgi_result = otr_tgi.solve(
        &references,
        &otr_values,
        &step_func).unwrap();

    //save recovered mask data in csv.
    let mut save_path = std::path::PathBuf::from(&args.dir);
    save_path.push(&args.fname);
    csv_converter::save_1darray(&otr_tgi_result, &save_path.to_string_lossy())?;

    //plot mask data
    let mut plot_file = std::path::PathBuf::from(&args.dir);
    plot_file.push(save_path.file_stem().unwrap().to_str().unwrap().to_owned() + "_plotted.png");
    draw_1darrays::draw_1darrays(
        &[(otr_tgi_result, RED)],
        &plot_file.to_string_lossy(),
        &args.fname,
    );

    Ok(())
}
