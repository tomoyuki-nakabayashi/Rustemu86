//! Exceptions occur in the CPU.
use crate::decode::DecodeError;
use crate::execute::ExecuteError;
use crate::fetch::FetchError;
use crate::lsu::LsuError;

/// This is just an wrapper of each stage exception.
#[derive(Debug, Fail, PartialEq)]
pub enum InternalExceptions {
    #[fail(display = "{}", error)]
    FetchException { error: FetchError },

    #[fail(display = "{}", error)]
    DecodeException { error: DecodeError },

    #[fail(display = "{}", error)]
    ExecuteException { error: ExecuteError },

    #[fail(display = "{}", error)]
    MemoryAccessException { error: LsuError },
}

impl From<FetchError> for InternalExceptions {
    fn from(error: FetchError) -> InternalExceptions {
        InternalExceptions::FetchException { error }
    }
}

impl From<DecodeError> for InternalExceptions {
    fn from(error: DecodeError) -> InternalExceptions {
        InternalExceptions::DecodeException { error }
    }
}

impl From<ExecuteError> for InternalExceptions {
    fn from(error: ExecuteError) -> InternalExceptions {
        InternalExceptions::ExecuteException { error }
    }
}

impl From<LsuError> for InternalExceptions {
    fn from(error: LsuError) -> InternalExceptions {
        InternalExceptions::MemoryAccessException { error }
    }
}
