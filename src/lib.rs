use rand::{distributions::Alphanumeric, thread_rng, Rng};

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

/// Generate random passwords.
///
/// `option` sets the password type. It can be `pin`, `alpha` or `full`. `length` sets the password length.
///
/// # Examples
///
/// ```
/// use pwgen::generate;
///
/// let pw = generate("pin", 8);
///
/// assert_eq!(pw.len(), 8);
/// assert!(pw.chars().all(|ch| ch.is_ascii_digit()));
/// ```
pub fn generate(option: &str, length: u8) -> String {
    match option {
        "pin" => pin(length),
        "full" => everything(length),
        &_ => alphanum(length),
    }
}

#[cfg(test)]
mod tests;
