#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
mod authentication;
mod protocol;
mod send_key;
#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "tokio")]
pub use crate::tokio::*;
pub use authentication::*;
pub use send_key::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
