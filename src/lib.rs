pub mod result;

#[cfg(feature = "std_err")]
pub mod std_err;

#[cfg(feature = "csv_err")]
pub mod csv_err;

#[cfg(feature = "axum_err")]
pub mod axum_err;
