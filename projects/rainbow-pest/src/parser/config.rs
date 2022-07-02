use std::ops::Range;

use pest::iterators::Pair;

use crate::Rule;

///
pub struct ParserConfig {}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {}
    }
}

impl ParserConfig {
    pub fn get_position(&self, s: &Pair<Rule>) -> Range<u32> {
        let us = s.as_span().start_pos();
        let es = s.as_span().end_pos();
        Range { start: us.pos() as u32, end: es.pos() as u32 }
    }
}
