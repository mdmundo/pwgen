//! Auxiliary functions to generate passwords.
use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Returns password containing alphanumeric chars.
fn alphanum(length: u8) -> String {
    let mut rng = thread_rng();
    (0..length)
        .filter_map(|_| -> Option<char> { rng.sample(Alphanumeric).try_into().ok() })
        .collect()
}

/// Returns password containing ascii chars ranging from '0' to '9'.
/// In other words, digits.
fn pin(length: u8) -> String {
    let mut rng = thread_rng();
    (0..length)
        .filter_map(|_| -> Option<char> { rng.gen_range(b'0'..=b'9').try_into().ok() })
        .collect()
}

/// Returns password containing ascii chars ranging from '!' to '~'.
/// In other words, alphanum + symbols.
fn everything(length: u8) -> String {
    let mut rng = thread_rng();
    (0..length)
        .filter_map(|_| -> Option<char> { rng.gen_range(b'!'..=b'~').try_into().ok() })
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
#[inline]
#[must_use]
pub fn generate(option: &str, length: u8) -> String {
    match option {
        "pin" => pin(length),
        "full" => everything(length),
        &_ => alphanum(length),
    }
}

#[cfg(test)]
mod tests;
