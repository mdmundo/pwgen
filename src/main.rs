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
        copy(pw);
    }
}

fn copy(pw: String) {
    if let Ok(mut clipboard) = Clipboard::new() {
        if clipboard.set_text(pw).is_err() {
            panic!("Could not place the password onto the clipboard");
        }
    } else {
        panic!("Could not create an instance of the clipboard");
    }
}
