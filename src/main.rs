use structopt::StructOpt;

/// Generate passwords from the command line
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

    /// Set password width [max: 255]
    #[structopt(short, long, default_value = "16")]
    width: u8,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
