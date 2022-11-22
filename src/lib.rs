#![allow(
    clippy::implicit_return,
    clippy::unseparated_literal_suffix,
    clippy::multiple_crate_versions
)]

//! Auxiliary functions to generate passwords.
use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[cfg(test)]
mod tests;

/// Returns password containing alphanumeric chars.
fn alphanum(length: usize) -> String {
    let mut rng = thread_rng();
    core::iter::repeat_with(|| char::from(rng.sample(Alphanumeric)))
        .take(length)
        .collect()
}

/// Returns password containing ascii chars ranging from '0' to '9'.
/// In other words, digits.
fn pin(length: usize) -> String {
    let mut rng = thread_rng();
    core::iter::repeat_with(|| char::from(rng.gen_range(b'0'..=b'9')))
        .take(length)
        .collect()
}

/// Returns password containing ascii chars ranging from '!' to '~'.
/// In other words, alphanum + symbols.
fn everything(length: usize) -> String {
    let mut rng = thread_rng();
    core::iter::repeat_with(|| char::from(rng.gen_range(b'!'..=b'~')))
        .take(length)
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
pub fn generate(option: &str, length: usize) -> String {
    match option {
        "pin" => pin(length),
        "full" => everything(length),
        &_ => alphanum(length),
    }
}
