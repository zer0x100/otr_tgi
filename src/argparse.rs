//! # Arg Parse
//! define a structure for command line argument parse.
//! the structure provides how-to-use message of 'otr-tgi'
use clap::Parser;

///OTR TGI
///
/// 'otr-tgi' calucalute otr-tgi(one time read time domain ghost imaging) result.
#[derive(Parser)]
pub struct Args {
    /// The number of test samples.
    #[arg(long)]
    pub sample_size: usize,
    /// Otr point.
    #[arg(long)]
    pub otr_point: usize,
    /// Reference signals file(csv)
    ///
    /// 2d csv file required.(i, j) is a value of j-th time in i-th sample.
    #[arg(long)]
    pub references: String,
    /// Otr values file.
    ///
    /// 1d csv file required.
    /// (0, i) is a value of the otr point in the i-th sample.
    #[arg(long)]
    pub otr_values: String,
    ///Slow Step Function File
    ///
    /// 1d csv file. (0, i) is the value of i-th time in slow step function used for the otr tgi.
    #[arg(long)]
    pub step_func: String,
    ///file name to save recovered mask data in. its extension need to be .csv.
    #[arg(long)]
    pub fname: String,
    /// directory to save mask file.
    #[arg(long)]
    pub dir: String,
}
