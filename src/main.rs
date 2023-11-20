//! # OTR TGI
//!
//! 'otr-tgi' is for otr-tgi(one time read time domain ghost imaging).
//! <otr-tgi> --help command shows short description and how to use of this program.
mod argparse;
mod otr_tgi_alg;

mod prelude {
    pub use ndarray::prelude::*;
    pub use anyhow::{anyhow, Result};
}

use clap::Parser;

fn main() {
    let args = argparse::Args::parse();
}
