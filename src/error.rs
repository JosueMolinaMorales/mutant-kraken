use std::io;

pub type Result<T> = std::result::Result<T, KodeKrakenError>;

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    #[test]
    fn test_display_file_reading_error() {
        let error = KodeKrakenError::FileReadingError("Failed to read file".to_string());
        let expected_output = "Error while reading file: Failed to read file";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_mutation_generation_error() {
        let error = KodeKrakenError::MutationGenerationError;
        let expected_output = "Error while generating mutation";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_mutation_gathering_error() {
        let error = KodeKrakenError::MutationGatheringError;
        let expected_output = "Error while gathering mutations";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_mutation_build_test_error() {
        let error = KodeKrakenError::MutationBuildTestError;
        let expected_output = "Error while building and testing";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_conversion_error() {
        let error = KodeKrakenError::ConversionError;
        let expected_output = "Error while converting";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_generic_error() {
        let error = KodeKrakenError::Error("Something went wrong".to_string());
        let expected_output = "Error: Something went wrong";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let converted_error: KodeKrakenError = io_error.into();
        let expected_error = KodeKrakenError::Error("File not found".to_string());
        assert_eq!(converted_error, expected_error);
    }

    #[test]
    fn test_result_conversion_ok() {
        let result: Result<i32> = Ok(42);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_conversion_err() {
        let result: Result<i32> = Err(KodeKrakenError::Error("Something went wrong".to_string()));
        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.unwrap_err(),
            KodeKrakenError::Error("Something went wrong".to_string())
        );
    }
}
