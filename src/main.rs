use pwgen::parse_option;
use structopt::StructOpt;

/// Generate passwords from the command line
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Set type of password: `pin`, `alpha` or `full`
    #[structopt(short, long, default_value = "alpha")]
    r#type: String,

    /// Set password length [max: 255]
    #[structopt(short, long, default_value = "16")]
    length: u8,
}

fn main() {
    let opt = Opt::from_args();

    println!("{}", parse_option(opt.r#type.as_str(), opt.length));
}
