//! # OTR TGI
//!
//! 'otr-tgi' is for otr-tgi(one time read time domain ghost imaging).
//! <otr-tgi> --help command shows short description and how to use of this program.
mod argparse;
mod csv_converter;
mod otr_tgi_alg;

mod prelude {
    pub use ndarray::prelude::*;
    pub use ndarray_linalg::Norm;
    pub use std::error::Error;
}

use crate::prelude::*;
use clap::Parser;
use otr_tgi_alg::OTRTGI;

fn main() -> Result<(), Box<dyn Error>> {
    //parse command line arguments.
    let args = argparse::Args::parse();

    //convert csv data to String
    let references = csv_converter::csv_to_2darray(
        &std::fs::read_to_string(args.references)?,
        (args.sample_size, args.otr_point+1),
    )?;
    let otr_values = csv_converter::csv_to_1darray(
        &std::fs::read_to_string(args.otr_values)?,
        args.sample_size,
    )?;
    if otr_values.len() < args.sample_size {
        return Err(From::from(
            "the number of otr_values is less than sample size/",
        ));
    }
    let step_func =
        csv_converter::csv_to_1darray(&std::fs::read_to_string(args.step_func)?, args.otr_point+1)?;

    let otr_tgi = otr_tgi_alg::normal::OTRTGINormal::new();
    let otr_tgi_result = otr_tgi.solve(
        &references,
        &otr_values,
        &step_func).unwrap();

    let mut save_path = std::path::PathBuf::from(&args.dir);
    save_path.push(args.fname);
    csv_converter::save_1darray(&otr_tgi_result, &save_path.to_string_lossy())?;

    Ok(())
}
