#[derive(Debug, thiserror::Error)]
pub enum SolanaError {
    #[error(transparent)]
    ClientError(#[from] solana_client::client_error::ClientError),

    #[error(transparent)]
    ProgramError(#[from] solana_sdk::program_error::ProgramError),
}
