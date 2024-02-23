#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("The provided file could not be found on disk")]
    FileNotFound,
    #[error("The provided file is not a valid CBL file, or is malformed: {0}")]
    InvalidCBL(String),
}

pub type CLIResult<T> = Result<T, CLIError>;
