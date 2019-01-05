//! Errors.

pub type Result<T> = std::result::Result<T, LoaderError>;

/// Errors occur in loader.
#[derive(Debug, Fail)]
pub enum LoaderError {
    // errors caused by file or mmap operations.
    #[fail(display = "{}", error)]
    FileOperationError { error: std::io::Error },

    #[fail(display = "fail to read elf header magic!")]
    InvalidElfFormat,

    #[fail(display = "lack of enough length data!")]
    TooShortBinary,
}

impl From<std::io::Error> for LoaderError {
    fn from(error: std::io::Error) -> LoaderError {
        LoaderError::FileOperationError { error }
    }
}
