#![warn(missing_docs)]

//! missing

mod ast;
#[cfg(feature = "serde")]
mod serde;
mod traits;
pub mod utils;
pub mod value;
#[macro_use]
mod macros;

pub use self::ast::{ASTKind, ASTNode};
pub use voml_error::{VomlError,VomlErrorKind,Result};
pub use self::value::Value;
#[cfg(feature = "serde")]
pub use self::serde::ReadableConfigSerializer;

/// if ture, { } will be null
pub const BUILD_EMPTY_SCOPE: bool = true;
