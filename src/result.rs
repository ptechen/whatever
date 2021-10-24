#[cfg(feature = "actix_err")]
use actix_web;
#[cfg(feature = "chrono_err")]
use chrono;
#[cfg(feature = "csv_err")]
use csv;
#[cfg(feature = "csv_err")]
use csv::Writer;
use enum_repr::EnumRepr;
use libc::*;
#[cfg(feature = "rbatis_err")]
use rbatis;
#[cfg(feature = "reqwest_err")]
use reqwest;
use serde::Serialize;
#[cfg(feature = "serde_json_err")]
use serde_json;
#[cfg(feature = "sqlx_err")]
use sqlx;
use std;
use std::fmt::Display;
use std::fmt::Formatter;
#[cfg(feature = "zip_err")]
use zip;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "std_err")]
    ParseIntError(std::num::ParseIntError),
    #[cfg(feature = "std_err")]
    Utf8Error(std::str::Utf8Error),
    #[cfg(feature = "std_err")]
    IoError(std::io::Error),
    #[cfg(feature = "rbatis_err")]
    RbatisError(rbatis::Error),
    #[cfg(feature = "actix_err")]
    ActixError(actix_web::error::Error),
    #[cfg(feature = "reqwest_err")]
    ReqwestError(reqwest::Error),
    #[cfg(feature = "serde_json_err")]
    SerdeJsonError(serde_json::Error),
    #[cfg(feature = "clickhouse_err")]
    ClickHouseError(clickhouse::error::Error),
    #[cfg(feature = "csv_err")]
    CsvError(csv::Error),
    #[cfg(feature = "csv_err")]
    CsvIntoInnerError(csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>),
    #[cfg(feature = "std_err")]
    StdIntoInnerError(std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),
    #[cfg(feature = "zip_err")]
    ZipError(zip::result::ZipError),
    #[cfg(feature = "chrono_err")]
    ChronoParseError(chrono::ParseError),
    #[cfg(feature = "sqlx_err")]
    SqlxError(sqlx::Error),
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
    SqlxErrorCode = 523,
}

#[derive(Debug, Clone, Default)]
pub struct CustomError {
    pub message: String,
}

impl From<String> for CustomError {
    fn from(message: String) -> CustomError {
        CustomError { message }
    }
}

impl<'a> From<CustomError> for &'a dyn std::error::Error {
    fn from(error: CustomError) -> &'a dyn std::error::Error {
        error.into()
    }
}

impl std::error::Error for CustomError {}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            #[cfg(feature = "std_err")]
            Error::IoError(ref e) => Some(e),
            #[cfg(feature = "std_err")]
            Error::Utf8Error(ref e) => Some(e),
            #[cfg(feature = "std_err")]
            Error::ParseIntError(ref e) => Some(e),
            #[cfg(feature = "rbatis_err")]
            Error::RbatisError(ref e) => Some(e),
            #[cfg(feature = "actix_err")]
            Error::ActixError(ref e) => Some(e),
            #[cfg(feature = "reqwest_err")]
            Error::ReqwestError(ref e) => Some(e),
            #[cfg(feature = "serde_json_err")]
            Error::SerdeJsonError(ref e) => Some(e),
            #[cfg(feature = "clickhouse_err")]
            Error::ClickHouseError(ref e) => Some(e),
            #[cfg(feature = "zip_err")]
            Error::ZipError(ref e) => Some(e),
            #[cfg(feature = "csv_err")]
            Error::CsvError(ref e) => Some(e),
            #[cfg(feature = "csv_err")]
            Error::CsvIntoInnerError(ref e) => Some(e),
            #[cfg(feature = "std_err")]
            Error::StdIntoInnerError(ref e) => Some(e),
            #[cfg(feature = "chrono_err")]
            Error::ChronoParseError(ref e) => Some(e),
            #[cfg(feature = "sqlx_err")]
            Error::SqlxError(ref e) => Some(e),
            Error::CustomError(ref e) => {
                let s: CustomError = From::from(e.to_string());
                Some(s.into())
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            #[cfg(feature = "std_err")]
            Error::IoError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            Error::Utf8Error(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            Error::ParseIntError(ref e) => e.fmt(f),
            #[cfg(feature = "rbatis_err")]
            Error::RbatisError(ref e) => e.fmt(f),
            #[cfg(feature = "actix_err")]
            Error::ActixError(ref e) => e.fmt(f),
            #[cfg(feature = "reqwest_err")]
            Error::ReqwestError(ref e) => e.fmt(f),
            #[cfg(feature = "serde_json_err")]
            Error::SerdeJsonError(ref e) => e.fmt(f),
            #[cfg(feature = "clickhouse_err")]
            Error::ClickHouseError(ref e) => e.fmt(f),
            #[cfg(feature = "zip_err")]
            Error::ZipError(ref e) => e.fmt(f),
            #[cfg(feature = "csv_err")]
            Error::CsvError(ref e) => e.fmt(f),
            #[cfg(feature = "csv_err")]
            Error::CsvIntoInnerError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            Error::StdIntoInnerError(ref e) => e.fmt(f),
            #[cfg(feature = "chrono_err")]
            Error::ChronoParseError(ref e) => e.fmt(f),
            #[cfg(feature = "sqlx_err")]
            Error::SqlxError(ref e) => e.fmt(f),
            Error::CustomError(ref e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std_err")]
impl From<std::num::ParseIntError> for Error {
    fn from(s: std::num::ParseIntError) -> Self {
        Error::ParseIntError(s)
    }
}

#[cfg(feature = "std_err")]
impl From<std::io::Error> for Error {
    fn from(s: std::io::Error) -> Self {
        Error::IoError(s)
    }
}

#[cfg(feature = "std_err")]
impl From<std::str::Utf8Error> for Error {
    fn from(s: std::str::Utf8Error) -> Self {
        Error::Utf8Error(s)
    }
}

#[cfg(feature = "rbatis_err")]
impl From<rbatis::Error> for Error {
    fn from(s: rbatis::Error) -> Self {
        Error::RbatisError(s)
    }
}

#[cfg(feature = "actix_err")]
impl From<actix_web::error::Error> for Error {
    fn from(s: actix_web::error::Error) -> Self {
        Error::ActixError(s)
    }
}

#[cfg(feature = "reqwest_err")]
impl From<reqwest::Error> for Error {
    fn from(s: reqwest::Error) -> Self {
        Error::ReqwestError(s)
    }
}

#[cfg(feature = "serde_json_err")]
impl From<serde_json::Error> for Error {
    fn from(s: serde_json::Error) -> Self {
        Error::SerdeJsonError(s)
    }
}

#[cfg(feature = "clickhouse_err")]
impl From<clickhouse::error::Error> for Error {
    fn from(s: clickhouse::error::Error) -> Self {
        Error::ClickHouseError(s)
    }
}

#[cfg(feature = "csv_err")]
impl From<csv::Error> for Error {
    fn from(s: csv::Error) -> Self {
        Error::CsvError(s)
    }
}

#[cfg(feature = "csv_err")]
impl From<csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>> for Error {
    fn from(s: csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>) -> Self {
        Error::CsvIntoInnerError(s)
    }
}

#[cfg(feature = "std_err")]
impl From<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> for Error {
    fn from(s: std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>) -> Self {
        Error::StdIntoInnerError(s)
    }
}

#[cfg(feature = "zip_err")]
impl From<zip::result::ZipError> for Error {
    fn from(s: zip::result::ZipError) -> Self {
        Error::ZipError(s)
    }
}

#[cfg(feature = "chrono_err")]
impl From<chrono::ParseError> for Error {
    fn from(s: chrono::ParseError) -> Self {
        Error::ChronoParseError(s)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::CustomError(s)
    }
}

#[cfg(feature = "sqlx_err")]
impl From<sqlx::Error> for Error {
    fn from(s: sqlx::Error) -> Self {
        Error::SqlxError(s)
    }
}
