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
    pub sample_size: Option<usize>,
    /// Reference signals file(csv)
    /// 
    /// 2d csv file required.(i, j) is a value of j-th time in i-th sample.
    #[arg(long)]
    pub references: String,
    /// Otr point and values file.
    /// 
    /// (1 x sample_size + 1) 1d csv file required.
    /// (0, 0) is otr point. if i>=0, (0, i+1) is a value of otr_point in i-th sample.
    #[arg(long)]
    pub otr_point_values: String,
}
