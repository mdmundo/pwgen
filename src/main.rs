use std::path::PathBuf;
use structopt::StructOpt;

/// Generate passwords
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Include numbers
    #[structopt(short, long)]
    number: bool,

    /// Include lowercase letters
    #[structopt(short, long)]
    lower: bool,

    /// Include uppercase letters
    #[structopt(short, long)]
    upper: bool,

    /// Include symbols
    #[structopt(short, long)]
    symbol: bool,

    /// Set password length [max: 255] [min: 8]
    #[structopt(short, long, default_value = "16")]
    range: u8,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
