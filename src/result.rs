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
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseIntError;
use std::str::Utf8Error;
#[cfg(feature = "zip_err")]
use zip;

pub type AppResult<Result> = std::result::Result<Result, AppError>;

#[derive(Debug)]
pub enum AppError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
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
    #[cfg(feature = "csv_err")]
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
            #[cfg(feature = "rbatis_err")]
            AppError::RbatisError(ref e) => Some(e),
            #[cfg(feature = "actix_err")]
            AppError::ActixError(ref e) => Some(e),
            #[cfg(feature = "reqwest_err")]
            AppError::ReqwestError(ref e) => Some(e),
            #[cfg(feature = "serde_json_err")]
            AppError::SerdeJsonError(ref e) => Some(e),
            #[cfg(feature = "clickhouse_err")]
            AppError::ClickHouseError(ref e) => Some(e),
            #[cfg(feature = "zip_err")]
            AppError::ZipError(ref e) => Some(e),
            #[cfg(feature = "csv_err")]
            AppError::CsvError(ref e) => Some(e),
            #[cfg(feature = "csv_err")]
            AppError::CsvIntoInnerError(ref e) => Some(e),
            #[cfg(feature = "csv_err")]
            AppError::StdIntoInnerError(ref e) => Some(e),
            #[cfg(feature = "chrono_err")]
            AppError::ChronoParseError(ref e) => Some(e),
            #[cfg(feature = "sqlx_err")]
            AppError::SqlxError(ref e) => Some(e),
            AppError::CustomError(ref e) => {
                let s: CustomError = From::from(e.to_string());
                Some(s.into())
            }
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AppError::IoError(ref e) => e.fmt(f),
            AppError::Utf8Error(ref e) => e.fmt(f),
            AppError::ParseIntError(ref e) => e.fmt(f),
            #[cfg(feature = "rbatis_err")]
            AppError::RbatisError(ref e) => e.fmt(f),
            #[cfg(feature = "actix_err")]
            AppError::ActixError(ref e) => e.fmt(f),
            #[cfg(feature = "reqwest_err")]
            AppError::ReqwestError(ref e) => e.fmt(f),
            #[cfg(feature = "serde_json_err")]
            AppError::SerdeJsonError(ref e) => e.fmt(f),
            #[cfg(feature = "clickhouse_err")]
            AppError::ClickHouseError(ref e) => e.fmt(f),
            #[cfg(feature = "zip_err")]
            AppError::ZipError(ref e) => e.fmt(f),
            #[cfg(feature = "csv_err")]
            AppError::CsvError(ref e) => e.fmt(f),
            #[cfg(feature = "csv_err")]
            AppError::CsvIntoInnerError(ref e) => e.fmt(f),
            #[cfg(feature = "csv_err")]
            AppError::StdIntoInnerError(ref e) => e.fmt(f),
            #[cfg(feature = "chrono_err")]
            AppError::ChronoParseError(ref e) => e.fmt(f),
            #[cfg(feature = "sqlx_err")]
            AppError::SqlxError(ref e) => e.fmt(f),
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

#[cfg(feature = "rbatis_err")]
impl From<rbatis::Error> for AppError {
    fn from(s: rbatis::Error) -> Self {
        AppError::RbatisError(s)
    }
}

#[cfg(feature = "actix_err")]
impl From<actix_web::error::Error> for AppError {
    fn from(s: actix_web::error::Error) -> Self {
        AppError::ActixError(s)
    }
}

#[cfg(feature = "reqwest_err")]
impl From<reqwest::Error> for AppError {
    fn from(s: reqwest::Error) -> Self {
        AppError::ReqwestError(s)
    }
}

#[cfg(feature = "serde_json_err")]
impl From<serde_json::Error> for AppError {
    fn from(s: serde_json::Error) -> Self {
        AppError::SerdeJsonError(s)
    }
}

#[cfg(feature = "clickhouse_err")]
impl From<clickhouse::error::Error> for AppError {
    fn from(s: clickhouse::error::Error) -> Self {
        AppError::ClickHouseError(s)
    }
}

#[cfg(feature = "csv_err")]
impl From<csv::Error> for AppError {
    fn from(s: csv::Error) -> Self {
        AppError::CsvError(s)
    }
}

#[cfg(feature = "csv_err")]
impl From<csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>> for AppError {
    fn from(s: csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>) -> Self {
        AppError::CsvIntoInnerError(s)
    }
}

#[cfg(feature = "csv_err")]
impl From<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> for AppError {
    fn from(s: std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>) -> Self {
        AppError::StdIntoInnerError(s)
    }
}

#[cfg(feature = "zip_err")]
impl From<zip::result::ZipError> for AppError {
    fn from(s: zip::result::ZipError) -> Self {
        AppError::ZipError(s)
    }
}

#[cfg(feature = "chrono_err")]
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

#[cfg(feature = "sqlx_err")]
impl From<sqlx::Error> for AppError {
    fn from(s: sqlx::Error) -> Self {
        AppError::SqlxError(s)
    }
}
