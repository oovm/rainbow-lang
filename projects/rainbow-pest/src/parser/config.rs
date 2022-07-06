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
    pub fn get_position(&self, s: &Pair<Rule>) -> Range<(u32, u32)> {
        let us = s.as_span().start_pos().line_col();
        let es = s.as_span().end_pos().line_col();
        Range { start: (us.0 as u32, us.1 as u32), end: (es.0 as u32, es.1 as u32) }
    }
}
