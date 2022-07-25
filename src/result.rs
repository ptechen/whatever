#[cfg(feature = "chrono_err")]
use chrono;
#[cfg(feature = "csv_err")]
use csv;
#[cfg(feature = "csv_err")]
use csv::Writer;
#[cfg(feature = "rbatis_err")]
use rbatis;
#[cfg(feature = "reqwest_err")]
use reqwest;
use serde::Serialize;
#[cfg(feature = "serde_json_err")]
use serde_json;
use std;
// use std::fmt;
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
    BoxError(#[from] Box<dyn std::error::Error>),

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

    #[cfg(feature = "solana_err")]
    #[error(transparent)]
    SolanaClientError(#[from] solana_client::client_error::ClientError),

    #[cfg(feature = "solana_err")]
    #[error(transparent)]
    SolanaProgramError(#[from] solana_sdk::program_error::ProgramError),

    #[cfg(feature = "solana_err")]
    #[error(transparent)]
    ParsePubkeyError(#[from] solana_sdk::pubkey::ParsePubkeyError),

    #[cfg(feature = "solana_err")]
    #[error(transparent)]
    RsaError(#[from] rsa::errors::Error),

    #[cfg(feature = "ed25519_dalek_err")]
    #[error(transparent)]
    SignatureError(#[from] ed25519_dalek::SignatureError),

    #[cfg(feature = "sqlx_err")]
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    CustomError(String),
}

#[repr(i64)]
#[derive(Serialize, Debug)]
pub enum AppErrorCode {
    Zero = 0,
    RequestErrorCode = 402,
    NotFound = 404,
    CustomErrorCode = 405,
    SystemErrorCode = 500,
    ParseIntErrorCode = 510,
    ParseFloatErrorCode = 511,
    Utf8ErrorCode = 512,
    IoErrorCode = 513,
    RbatisErrorCode = 514,
    ReqwestErrorCode = 515,
    SerdeJsonErrorCode = 516,
    ClickHouseErrorCode = 517,
    CsvErrorCode = 518,
    CsvIntoInnerErrorCode = 519,
    StdIntoInnerErrorCode = 520,
    ZipErrorCode = 521,
    ChronoParseErrorCode = 522,
    RedisErrorCode = 523,
    TaskJoinErrorCode = 524,
    AddrParseErrorCode = 525,
    SolanaClientErrorCode = 526,
    SolanaProgramErrorCode = 527,
    ParsePubkeyErrorCode = 528,
    SqlxErrorCode = 529,
    RsaErrorCode = 530,
    BoxErrorCode = 531,
    SignatureErrorCode= 532,
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
            AppError::BoxError(e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::IoError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::Utf8Error(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::ParseIntError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::ParseFloatError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::AddrParseError(ref e) => e.fmt(f),
            #[cfg(feature = "std_err")]
            AppError::StdIntoInnerError(ref e) => e.fmt(f),
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

            #[cfg(feature = "solana_err")]
            AppError::SolanaClientError(ref e) => e.fmt(f),
            #[cfg(feature = "solana_err")]
            AppError::SolanaProgramError(ref e) => e.fmt(f),
            #[cfg(feature = "solana_err")]
            AppError::ParsePubkeyError(ref e) => e.fmt(f),

            #[cfg(feature = "sqlx_err")]
            AppError::SqlxError(ref e) => e.fmt(f),
            #[cfg(feature = "rsa_err")]
            AppError::RsaError(e) => e.fmt(f),
            #[cfg(feature = "ed25519_dalek_err")]
            AppError::SignatureError(ref e) => e.fmt(f),

            AppError::CustomError(ref e) => e.fmt(f),
        }
    }
}

pub fn match_app_error(e: AppError) -> (AppErrorCode, String) {
    match e {
        #[cfg(feature = "std_err")]
        AppError::BoxError(e) => (AppErrorCode::BoxErrorCode, e.to_string()),
        #[cfg(feature = "std_err")]
        AppError::ParseIntError(e) => (AppErrorCode::ParseIntErrorCode, e.to_string()),
        #[cfg(feature = "std_err")]
        AppError::Utf8Error(e) => (AppErrorCode::Utf8ErrorCode, e.to_string()),
        #[cfg(feature = "std_err")]
        AppError::IoError(e) => (AppErrorCode::IoErrorCode, e.to_string()),
        #[cfg(feature = "std_err")]
        AppError::StdIntoInnerError(e) => (AppErrorCode::StdIntoInnerErrorCode, e.to_string()),
        #[cfg(feature = "std_err")]
        AppError::AddrParseError(e) => (AppErrorCode::AddrParseErrorCode, e.to_string()),
        #[cfg(feature = "std_err")]
        AppError::ParseFloatError(e) => (AppErrorCode::ParseFloatErrorCode, e.to_string()),
        #[cfg(feature = "serde_json_err")]
        AppError::SerdeJsonError(e) => (AppErrorCode::SerdeJsonErrorCode, e.to_string()),
        #[cfg(feature = "axum_err")]
        AppError::ReqwestError(e) => (AppErrorCode::ReqwestErrorCode, e.to_string()),
        #[cfg(feature = "axum_err")]
        AppError::ValidationError(e) => (AppErrorCode::RequestErrorCode, e.to_string()),
        #[cfg(feature = "axum_err")]
        AppError::AxumJsonRejection(e) => (AppErrorCode::RequestErrorCode, e.to_string()),

        #[cfg(feature = "chrono_err")]
        AppError::ChronoParseError(e) => (AppErrorCode::ChronoParseErrorCode, e.to_string()),

        #[cfg(feature = "redis_err")]
        AppError::RedisError(e) => (AppErrorCode::RedisErrorCode, e.to_string()),

        #[cfg(feature = "solana_err")]
        AppError::SolanaClientError(e) => (AppErrorCode::SolanaClientErrorCode, e.to_string()),
        #[cfg(feature = "solana_err")]
        AppError::SolanaProgramError(e) => (AppErrorCode::SolanaProgramErrorCode, e.to_string()),
        #[cfg(feature = "solana_err")]
        AppError::ParsePubkeyError(e) => (AppErrorCode::ParsePubkeyErrorCode, e.to_string()),
        #[cfg(feature = "rsa_err")]
        AppError::RsaError(e) => (AppErrorCode::RsaErrorCode, e.to_string()),
        #[cfg(feature = "sqlx_err")]
        AppError::SqlxError(e) => (AppErrorCode::SqlxErrorCode, e.to_string()),
        #[cfg(feature = "csv_err")]
        AppError::CsvError(e) => (AppErrorCode::CsvErrorCode, e.to_string()),
        #[cfg(feature = "csv_err")]
        AppError::CsvIntoInnerError(e) => (AppErrorCode::CsvIntoInnerErrorCode, e.to_string()),
        #[cfg(feature = "zip_err")]
        AppError::ZipError(e) => (AppErrorCode::ZipErrorCode, e.to_string()),
        #[cfg(feature = "task_join_err")]
        AppError::TaskJoinError(e) => (AppErrorCode::TaskJoinErrorCode, e.to_string()),
        #[cfg(feature = "rbatis_err")]
        AppError::RbatisError(e) => (AppErrorCode::RbatisErrorCode, e.to_string()),
        #[cfg(feature = "clickhouse_err")]
        AppError::ClickHouseError(e) => (AppErrorCode::ClickHouseErrorCode, e.to_string()),
        #[cfg(feature = "ed25519_dalek_err")]
        AppError::SignatureError(e) => (AppErrorCode::SignatureErrorCode, e.to_string()),
        AppError::CustomError(e) => (AppErrorCode::CustomErrorCode, e),
    }
}
