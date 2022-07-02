pub use pest::{
    self,
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
    prec_climber::{Assoc, Operator, PrecClimber}, Span,
};

pub use rainbow::{RainbowParser, Rule};

mod rainbow;
mod parser;
pub mod ast;
pub use parser::ParserConfig;