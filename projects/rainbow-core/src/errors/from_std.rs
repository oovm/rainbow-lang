use crate::RainbowError;
use std::fmt::Error;

impl From<Error> for RainbowError {
    fn from(e: Error) -> Self {
        RainbowError::runtime_error(e.to_string())
    }
}
