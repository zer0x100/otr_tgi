//! # OTR TGI
//!
//! 'otr-tgi' is for otr-tgi(one time read time domain ghost imaging).
//! <otr-tgi> --help command shows short description and how to use of this program.
mod argparse;

use clap::Parser;

fn main() {
    let args = argparse::Args::parse();
}
