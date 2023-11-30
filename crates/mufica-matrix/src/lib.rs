pub mod client;
pub mod config;
pub mod errors;
pub mod timeline;

pub use client::MatrixClient;
pub use config::MatrixConfig;
pub use errors::{Result, Error};
pub use timeline::MatrixTimeline;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
