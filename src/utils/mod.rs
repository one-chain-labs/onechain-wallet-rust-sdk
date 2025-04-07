//! Tool module, providing RSA signature and general utility functions.
//!
//! This module contains two main parts:
//! - RSA signature function: provided by `rsa_sign` submodule
//! - General utility functions: such as generating tracking IDs and random strings

pub mod rsa_sign;
pub mod zk_login;

use rand::{Rng, distributions::Alphanumeric};

/// Generate a new tracking ID.
///
/// The returned tracking ID format is `onechain-sdk-rust-{22-digit random string}`.
///
/// # Example
///
/// ```
/// use onechain_wallet_rust_sdk::utils::new_trace_id;
///
/// let trace_id = new_trace_id();
/// assert!(trace_id.starts_with("onechain-wallet-rust-sdk-"));
/// assert_eq!(trace_id.len(), 22 + "onechain-wallet-rust-sdk0=-".len());
/// ```
pub fn new_trace_id() -> String {
    format!("onechain-wallet-rust-sdk-{}", random_string(22))
}

/// Generates a random string of the specified length.
///
/// The generated string contains only letters and numbers.
///
/// # Parameters
///
/// * `length` - the length of the random string to be generated
///
/// # Example
///
/// ```
/// use onechain_wallet_rust_sdk::utils::random_string;
///
/// let random = random_string(10);
/// assert_eq!(random.len(), 10);
/// ```
pub fn random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}
