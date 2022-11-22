#![allow(
    clippy::implicit_return,
    clippy::unseparated_literal_suffix,
    clippy::multiple_crate_versions,
    clippy::print_stdout
)]

//! Generate passwords from the command line
use arboard::Clipboard;
use pwgen::generate;
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

    /// Do not not copy the password to the clipboard
    #[structopt(long = "no-copy")]
    do_not_copy: bool,
}

fn main() {
    let opt = Opt::from_args();

    let pw = generate(opt.r#type.as_str(), opt.length);
    println!("{}", &pw);

    if !opt.do_not_copy {
        if let Ok(mut clipboard) = Clipboard::new() {
            let _res = clipboard.set_text(pw);
        }
    }
}
