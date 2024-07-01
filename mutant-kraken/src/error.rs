use std::io;

pub type Result<T> = std::result::Result<T, MutantKrakenError>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MutantKrakenError {
    FileReadingError(String),
    MutationGenerationError,
    MutationGatheringError,
    MutationBuildTestError,
    ConversionError,
    Error(String),
}

impl std::fmt::Display for MutantKrakenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MutantKrakenError::FileReadingError(message) => {
                write!(f, "Error while reading file: {}", message)
            }
            MutantKrakenError::MutationGenerationError => {
                write!(f, "Error while generating mutation")
            }
            MutantKrakenError::MutationGatheringError => {
                write!(f, "Error while gathering mutations")
            }
            MutantKrakenError::MutationBuildTestError => {
                write!(f, "Error while building and testing")
            }
            MutantKrakenError::ConversionError => {
                write!(f, "Error while converting")
            }
            MutantKrakenError::Error(message) => write!(f, "Error: {}", message),
        }
    }
}

impl std::error::Error for MutantKrakenError {}

impl From<io::Error> for MutantKrakenError {
    fn from(error: io::Error) -> Self {
        MutantKrakenError::Error(error.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    #[test]
    fn test_display_file_reading_error() {
        let error = MutantKrakenError::FileReadingError("Failed to read file".to_string());
        let expected_output = "Error while reading file: Failed to read file";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_mutation_generation_error() {
        let error = MutantKrakenError::MutationGenerationError;
        let expected_output = "Error while generating mutation";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_mutation_gathering_error() {
        let error = MutantKrakenError::MutationGatheringError;
        let expected_output = "Error while gathering mutations";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_mutation_build_test_error() {
        let error = MutantKrakenError::MutationBuildTestError;
        let expected_output = "Error while building and testing";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_conversion_error() {
        let error = MutantKrakenError::ConversionError;
        let expected_output = "Error while converting";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_display_generic_error() {
        let error = MutantKrakenError::Error("Something went wrong".to_string());
        let expected_output = "Error: Something went wrong";
        assert_eq!(error.to_string(), expected_output);
    }

    #[test]
    fn test_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let converted_error: MutantKrakenError = io_error.into();
        let expected_error = MutantKrakenError::Error("File not found".to_string());
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
        let result: Result<i32> = Err(MutantKrakenError::Error("Something went wrong".to_string()));
        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.unwrap_err(),
            MutantKrakenError::Error("Something went wrong".to_string())
        );
    }
}
