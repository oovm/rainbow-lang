use crate::VomlError;
use globset::Error;

impl From<Error> for VomlError {
    fn from(e: Error) -> Self {
        VomlError::runtime_error(e.to_string())
    }
}
