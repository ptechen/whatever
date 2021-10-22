use enum_repr::EnumRepr;
use libc::*;
use csv::Writer;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::str::Utf8Error;
use rbatis;
use zip;

pub type AppResult<Result> = std::result::Result<Result, AppError>;

#[derive(Debug)]
pub enum AppError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
    RbatisError(rbatis::Error),
    ActixError(actix_web::error::Error),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    ClickHouseError(clickhouse::error::Error),
    CsvError(csv::Error),
    CsvIntoInnerError(csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>),
    StdIntoInnerError(std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),
    ZipError(zip::result::ZipError),
    ChronoParseError(chrono::ParseError),
    CustomError(String),
}

#[EnumRepr(type = "c_int")]
#[derive(Serialize, Debug)]
pub enum AppErrorCode {
    Zero = 0,
    RequestErrorCode = 402,
    NotFound = 404,
    CustomError = 405,
    SystemErrorCode = 500,
    ParseIntErrorCode = 510,
    Utf8ErrorCode = 511,
    IoErrorCode = 512,
    RbatisErrorCode = 513,
    ActixErrorCode = 514,
    ReqwestErrorCode = 515,
    SerdeJsonErrorCode = 516,
    ClickHouseErrorCode = 517,
    CsvErrorCode = 518,
    CsvIntoInnerErrorCode = 519,
    StdIntoInnerErrorCode = 520,
    ZipErrorCode = 521,
    ChronoParseErrorCode = 522,
}

#[derive(Debug, Clone, Default)]
pub struct CustomError {
    pub message: String,
}

impl From<String> for CustomError {
    fn from(message: String) -> CustomError {
        CustomError{message}
    }
}

impl<'a> From<CustomError> for &'a dyn Error {
    fn from(error: CustomError) -> &'a dyn Error {
        error.into()
    }
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            AppError::IoError(ref e) => Some(e),
            AppError::Utf8Error(ref e) => Some(e),
            AppError::ParseIntError(ref e) => Some(e),
            AppError::RbatisError(ref e) => Some(e),
            AppError::ActixError(ref e) => Some(e),
            AppError::ReqwestError(ref e) => Some(e),
            AppError::SerdeJsonError(ref e) => Some(e),
            AppError::ClickHouseError(ref e) => Some(e),
            AppError::CsvError(ref e) => Some(e),
            AppError::ZipError(ref e) => Some(e),
            AppError::CsvIntoInnerError(ref e) => Some(e),
            AppError::StdIntoInnerError(ref e) => Some(e),
            AppError::ChronoParseError(ref e) => Some(e),
            AppError::CustomError(ref e) => {
                let s: CustomError = From::from(e.to_string());
                Some(s.into())
            },
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AppError::IoError(ref e) => e.fmt(f),
            AppError::Utf8Error(ref e) => e.fmt(f),
            AppError::ParseIntError(ref e) => e.fmt(f),
            AppError::RbatisError(ref e) => e.fmt(f),
            AppError::ActixError(ref e) => e.fmt(f),
            AppError::ReqwestError(ref e) => e.fmt(f),
            AppError::SerdeJsonError(ref e) => e.fmt(f),
            AppError::ClickHouseError(ref e) => e.fmt(f),
            AppError::CsvError(ref e) => e.fmt(f),
            AppError::ZipError(ref e) => e.fmt(f),
            AppError::CsvIntoInnerError(ref e) => e.fmt(f),
            AppError::StdIntoInnerError(ref e) => e.fmt(f),
            AppError::ChronoParseError(ref e) => e.fmt(f),
            AppError::CustomError(ref e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(s: std::num::ParseIntError) -> Self {
        AppError::ParseIntError(s)
    }
}

impl From<std::io::Error> for AppError {
    fn from(s: std::io::Error) -> Self {
        AppError::IoError(s)
    }
}

impl From<Utf8Error> for AppError {
    fn from(s: std::str::Utf8Error) -> Self {
        AppError::Utf8Error(s)
    }
}

impl From<rbatis::Error> for AppError {
    fn from(s: rbatis::Error) -> Self {
        AppError::RbatisError(s)
    }
}

impl From<actix_web::error::Error> for AppError {
    fn from(s: actix_web::error::Error) -> Self {
        AppError::ActixError(s)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(s: reqwest::Error) -> Self {
        AppError::ReqwestError(s)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(s: serde_json::Error) -> Self {
        AppError::SerdeJsonError(s)
    }
}

impl From<clickhouse::error::Error> for AppError {
    fn from(s: clickhouse::error::Error) -> Self {
        AppError::ClickHouseError(s)
    }
}

impl From<csv::Error> for AppError {
    fn from(s: csv::Error) -> Self {
        AppError::CsvError(s)
    }
}

impl From<csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>> for AppError {
    fn from(s: csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>) -> Self {
        AppError::CsvIntoInnerError(s)
    }
}

impl From<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> for AppError {
    fn from(s: std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>) -> Self {
        AppError::StdIntoInnerError(s)
    }
}

impl From<zip::result::ZipError> for AppError {
    fn from(s: zip::result::ZipError) -> Self {
        AppError::ZipError(s)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(s: chrono::ParseError) -> Self {
        AppError::ChronoParseError(s)
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::CustomError(s)
    }
}