use html_parser::Error;

use crate::RainbowError;

impl From<Error> for RainbowError {
    fn from(e: Error) -> Self {
        match e {
            Error::Parsing(v) => RainbowError::syntax_error(v),
            Error::Cli(v) => RainbowError::runtime_error(v),
            Error::IO(_) => {
                unreachable!()
            }
            Error::Serde(_) => {
                unreachable!()
            }
        }
    }
}
