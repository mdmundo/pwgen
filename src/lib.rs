use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn alphanum(length: u8) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

fn pin(length: u8) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.gen_range(b'0'..=b'9') as char)
        .collect()
}

fn everything(length: u8) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| rng.gen_range(b'!'..=b'~') as char)
        .collect()
}

pub fn parse_option(option: &str, length: u8) -> String {
    match option {
        "pin" => pin(length),
        "full" => everything(length),
        &_ => alphanum(length),
    }
}

#[cfg(test)]
mod tests;
