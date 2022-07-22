#[derive(Debug, thiserror::Error)]
pub enum CsvError {
    #[error(transparent)]
    CsvError(#[from] csv::Error),

    #[error(transparent)]
    CsvIntoInnerError(#[from] csv::IntoInnerError<csv::Writer<std::io::BufWriter<Vec<u8>>>>),

    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
}
