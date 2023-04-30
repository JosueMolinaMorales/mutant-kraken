use std::io;

pub type Result<T> = std::result::Result<T, KodeKrakenError>;

#[derive(Debug)]
pub enum KodeKrakenError {
    FileReadingError(String),
    MutationGenerationError,
    MutationGatheringError,
    MutationBuildTestError,
    ConversionError,
    Error(String),
}

impl std::fmt::Display for KodeKrakenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KodeKrakenError::FileReadingError(message) => {
                write!(f, "Error while reading file: {}", message)
            }
            KodeKrakenError::MutationGenerationError => {
                write!(f, "Error while generating mutation")
            }
            KodeKrakenError::MutationGatheringError => {
                write!(f, "Error while gathering mutations")
            }
            KodeKrakenError::MutationBuildTestError => {
                write!(f, "Error while building and testing")
            }
            KodeKrakenError::ConversionError => {
                write!(f, "Error while converting")
            }
            KodeKrakenError::Error(message) => write!(f, "Error: {}", message),
        }
    }
}

impl std::error::Error for KodeKrakenError {}

impl From<io::Error> for KodeKrakenError {
    fn from(error: io::Error) -> Self {
        KodeKrakenError::Error(error.to_string())
    }
}
