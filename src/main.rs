//! # OTR TGI
//!
//! 'otr-tgi' is for otr-tgi(one time read time domain ghost imaging).
//! <otr-tgi> --help command shows short description and how to use of this program.
mod argparse;
mod otr_tgi_alg;

mod prelude {
    pub use ndarray::prelude::*;
    pub use std::error::Error;
}

use crate::prelude::*;
use clap::Parser;

fn main() {
    //parse command line arguments.
    let args = argparse::Args::parse();

    //convert csv data to String
    let references_str = std::fs::read_to_string(args.references).unwrap();
    let otr_values_str = std::fs::read_to_string(args.otr_point_values).unwrap();
    let step_func_str = std::fs::read_to_string(args.step_func).unwrap();
}

fn csv_to_1darray(file: &str) -> Result<Array1<f64>, csv::Error> {
    let csv_str = std::fs::read_to_string(file).unwrap();
    let mut reader = csv::Reader::from_reader(csv_str.as_bytes());
    let mut signal = Vec::<f64>::new();
    for value in reader.headers()? {
        signal.push(value.parse::<f64>().unwrap());
    }

    Ok(Array::from(signal))
}

