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
use std;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use thiserror::Error;
#[cfg(feature = "task_join_err")]
use tokio;
#[cfg(feature = "zip_err")]
use zip;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[cfg(feature = "std_err")]
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[cfg(feature = "rbatis_err")]
    #[error(transparent)]
    RbatisError(#[from] rbatis::Error),

    #[cfg(feature = "reqwest_err")]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "serde_json_err")]
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[cfg(feature = "clickhouse_err")]
    #[error(transparent)]
    ClickHouseError(#[from] clickhouse::error::Error),

    #[cfg(feature = "csv_err")]
    #[error(transparent)]
    CsvError(#[from] csv::Error),

    #[cfg(feature = "csv_err")]
    #[error(transparent)]
    CsvIntoInnerError(#[from] csv::IntoInnerError<Writer<std::io::BufWriter<Vec<u8>>>>),

    #[cfg(feature = "std_err")]
    #[error(transparent)]
    StdIntoInnerError(#[from] std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),

    #[cfg(feature = "zip_err")]
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),

    #[cfg(feature = "chrono_err")]
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),

    #[cfg(feature = "axum_err")]
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[cfg(feature = "axum_err")]
    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[cfg(feature = "redis_err")]
    #[error(transparent)]
    RedisError(#[from] r2d2_redis::redis::RedisError),

    #[cfg(feature = "task_join_err")]
    #[error(transparent)]
    TaskJoinError(#[from] tokio::task::JoinError),

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
    ReqwestErrorCode = 514,
    SerdeJsonErrorCode = 515,
    ClickHouseErrorCode = 516,
    CsvErrorCode = 517,
    CsvIntoInnerErrorCode = 518,
    StdIntoInnerErrorCode = 519,
    ZipErrorCode = 520,
    ChronoParseErrorCode = 521,
    RedisErrorCode = 522,
    TaskJoinErrorCode = 523,
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

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            #[cfg(feature = "std_err")]
            AppError::IoError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::Utf8Error(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::ParseIntError(ref e) => e.fmt(f),
            #[cfg(feature = "rbatis_err")]
            AppError::RbatisError(ref e) => e.fmt(f),
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
            #[cfg(feature = "std_err")]
            AppError::StdIntoInnerError(ref e) => e.fmt(f),
            #[cfg(feature = "chrono_err")]
            AppError::ChronoParseError(ref e) => e.fmt(f),
            #[cfg(feature = "axum_err")]
            AppError::ValidationError(ref e) => e.fmt(f),
            #[cfg(feature = "axum_err")]
            AppError::AxumJsonRejection(ref e) => e.fmt(f),
            #[cfg(feature = "redis_err")]
            AppError::RedisError(ref e) => e.fmt(f),
            #[cfg(feature = "task_join_err")]
            AppError::TaskJoinError(ref e) => e.fmt(f),
            AppError::CustomError(ref e) => e.fmt(f),
        }
    }
}
