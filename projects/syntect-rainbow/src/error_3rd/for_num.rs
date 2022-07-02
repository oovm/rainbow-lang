use crate::VomlError;
use num::{bigint::TryFromBigIntError, BigInt};
use rust_decimal::Error;

impl From<TryFromBigIntError<BigInt>> for VomlError {
    fn from(e: TryFromBigIntError<BigInt>) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}

impl From<Error> for VomlError {
    fn from(e: Error) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}
