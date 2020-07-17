use std::error::Error as StdError;
use std::fmt;
use std::io;

use crate::remount::RemountError;
use crate::{Error, MountError, OSError};

impl OSError {
    /// Convert error to the one providing extra useful information
    pub fn explain(self) -> Error {
        let text = self.1.explain();
        match self.0 {
            MountError::Io(e) => Error(self.1, e, text),
            MountError::Remount(RemountError::Io(msg, io_err)) => {
                Error(self.1, io_err, format!("{}, {}", msg, text))
            }
            MountError::Remount(err) => {
                let text = format!("{}, {}", &err, text);
                let err = Box::new(err);
                Error(
                    self.1,
                    io::Error::new(io::ErrorKind::InvalidData, err),
                    text,
                )
            }
        }
    }
}

impl fmt::Display for OSError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.1, self.0)
    }
}

impl StdError for OSError {
    fn cause(&self) -> Option<&dyn StdError> {
        Some(&self.0)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {} ({})", self.0, self.1, self.2)
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        Some(&self.1)
    }
}
