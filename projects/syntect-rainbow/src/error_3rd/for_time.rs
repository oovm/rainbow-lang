use crate::VomlError;
use chrono::ParseError;

impl From<ParseError> for VomlError {
    fn from(e: ParseError) -> Self {
        VomlError::syntax_error(e.to_string())
    }
}
