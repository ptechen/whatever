#[cfg(feature = "chrono_err")]
use chrono;
#[cfg(feature = "rbatis_err")]
use rbatis;
#[cfg(feature = "reqwest_err")]
use reqwest;
use serde::Serialize;
#[cfg(feature = "serde_json_err")]
use serde_json;
use std;
use std::fmt::Display;
use std::fmt::Formatter;
#[cfg(feature = "task_join_err")]
use tokio;

#[cfg(feature = "std_err")]
use crate::std_err;

#[cfg(feature = "csv_err")]
use crate::csv_err;

#[cfg(feature = "axum_err")]
use crate::axum_err;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[cfg(feature = "std_err")]
    #[error(transparent)]
    StdError(#[from] std_err::StdError),

    #[cfg(feature = "csv_err")]
    #[error(transparent)]
    CsvError(#[from] csv_err::CsvError),

    #[cfg(feature = "axum_err")]
    #[error(transparent)]
    AxumError(#[from] axum_err::AxumError),

    #[cfg(feature = "sqlx_err")]
    #[error(transparent)]
    SqlxError(#[from] sqlx::error::Error),

    #[cfg(feature = "solana_sdk_err")]
    #[error(transparent)]
    ProgramError(#[from] solana_sdk::program_error::ProgramError),

    #[cfg(feature = "anchor_client_err")]
    #[error(transparent)]
    AnchorClientError(#[from] anchor_client::ClientError),

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

    #[cfg(feature = "chrono_err")]
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),

    #[cfg(feature = "redis_err")]
    #[error(transparent)]
    RedisError(#[from] r2d2_redis::redis::RedisError),

    #[cfg(feature = "task_join_err")]
    #[error(transparent)]
    TaskJoinError(#[from] tokio::task::JoinError),

    CustomError(String),
}

#[repr(i64)]
#[derive(Serialize, Debug)]
pub enum AppErrorCode {
    Zero = 0,
    RequestErrorCode = 402,
    NotFound = 404,
    CustomError = 405,
    SystemErrorCode = 500,
    StdErrorCode = 501,
    AxumErrorCode = 502,
    CsvErrorCode = 503,
    SqlxErrorCode = 504,
    RbatisErrorCode = 505,
    SolanaSdkErrorCode = 506,
    AnchorClientErrorCode = 507,
    ReqwestErrorCode = 508,
    SerdeJsonErrorCode = 509,
    ClickHouseErrorCode = 510,
    ChronoParseErrorCode = 511,
    RedisErrorCode = 512,
    TaskJoinErrorCode = 513,
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            #[cfg(feature = "std_err")]
            AppError::StdError(ref e) => e.fmt(f),
            #[cfg(feature = "csv_err")]
            AppError::CsvError(ref e) => e.fmt(f),
            #[cfg(feature = "axum_err")]
            AppError::AxumError(ref e) => e.fmt(f),
            #[cfg(feature = "sqlx_err")]
            AppError::SqlxError(ref e) => e.fmt(f),
            #[cfg(feature = "solana_sdk_err")]
            AppError::ProgramError(ref e) => e.fmt(f),
            #[cfg(feature = "anchor_client_err")]
            AppError::AnchorClientError(ref e) => e.fmt(f),
            #[cfg(feature = "rbatis_err")]
            AppError::RbatisError(ref e) => e.fmt(f),
            #[cfg(feature = "reqwest_err")]
            AppError::ReqwestError(ref e) => e.fmt(f),
            #[cfg(feature = "serde_json_err")]
            AppError::SerdeJsonError(ref e) => e.fmt(f),
            #[cfg(feature = "clickhouse_err")]
            AppError::ClickHouseError(ref e) => e.fmt(f),
            #[cfg(feature = "chrono_err")]
            AppError::ChronoParseError(ref e) => e.fmt(f),
            #[cfg(feature = "redis_err")]
            AppError::RedisError(ref e) => e.fmt(f),
            #[cfg(feature = "task_join_err")]
            AppError::TaskJoinError(ref e) => e.fmt(f),
            AppError::CustomError(ref e) => e.fmt(f),
        }
    }
}
