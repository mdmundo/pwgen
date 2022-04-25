use std::io::Stdin;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn alphanum(length: u8) -> String {
    let mut rng = thread_rng();
    let password: String = (0..(length as i32))
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    password
}

fn pin(length: u8) -> String {
    let mut rng = thread_rng();
    let mut pin = String::with_capacity(length as usize);
    for _ in 0..length {
        pin.push(rng.gen_range(b'0'..=b'9') as char);
    }

    pin
}

fn everything(length: u8) -> String {
    let mut rng = thread_rng();
    let mut password = String::with_capacity(length as usize);
    for _ in 0..length {
        password.push(rng.gen_range(b'!'..=b'~') as char);
    }

    password
}

pub fn parse_option(option: &str, length: u8) -> String {
    match option {
        "pin" => pin(length),
        "full" => everything(length),
        &_ => alphanum(length),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
