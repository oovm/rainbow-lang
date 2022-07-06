use crate::RainbowError;
use rainbow_pest::{Error, Rule};

impl From<Error<Rule>> for RainbowError {
    fn from(e: Error<Rule>) -> Self {
        todo!("{}", e)
    }
}
