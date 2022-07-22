use thiserror::Error;

#[derive(Debug, Error)]
pub enum StdError {
    #[cfg(feature = "std_err")]
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    StdIntoInnerError(#[from] std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),
}
