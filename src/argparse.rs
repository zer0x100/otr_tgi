//! # Arg Parse
//! define a structure for command line argument parse.
use clap::Parser;

#[derive(Parser)]
struct Args {
    dist: String,
    signal_size: usize,
    read_point: usize,
    test_size: usize, 
}