//! Support for connecting to ScyllaDB Serverless Cloud.

mod config;

pub use config::CloudConfig;
pub use config::CloudConfigError;
pub use config::CloudTlsProvider;

#[cfg(all(
    feature = "unstable-cloud",
    not(any(feature = "rustls-023", feature = "openssl-010"))
))]
compile_error!(
    r#""unstable-cloud" feature requires a TLS backend: at least one of ["rustls-023", "openssl-010"] is needed"#
);
