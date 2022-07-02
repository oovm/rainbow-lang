use pest::error::Error;
use pest::iterators::Pairs;
use pest::{state, Parser};

pub struct RainbowParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    EmptyLine,
    import_statement,
    schema_statement,
    meta_statement,
    language_statement,
    language_inherit,
    object_inherit,
    object,
    pair,
    list,
    Set,
    value,
    Special,
    Color,
    Number,
    SignedNumber,
    Decimal,
    DecimalBad,
    Integer,
    Sign,
    String,
    Character,
    namespace,
    SYMBOL,
    COMMENT,
    WHITESPACE,
    SEPARATOR,
}
#[allow(clippy::all)]
impl Parser<Rule> for RainbowParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> Result<Pairs<'i, Rule>, Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                use pest::Atomicity::NonAtomic;
                use pest::{ParseResult, ParserState};
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    if state.atomicity() == NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                use pest::Atomicity::{Atomic, CompoundAtomic};
                use pest::{ParseResult, ParserState};
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.sequence(|state| self::SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::statement(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    self::SEPARATOR(state).or_else(|state| state.sequence(|state| self::EmptyLine(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::EmptyLine(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::EmptyLine(state)))))))))).or_else(|state| self::schema_statement(state)).or_else(|state| self::language_statement(state)).or_else(|state| self::meta_statement(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyLine(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::EmptyLine, |state| state.atomic(Atomic, |state| state.sequence(|state| state.repeat(|state| self::WHITESPACE(state)).and_then(|state| self::NEWLINE(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn import_statement(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::import_statement, |state| state.sequence(|state| state.match_string("import").and_then(|state| super::hidden::skip(state)).and_then(|state| self::String(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn schema_statement(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::schema_statement, |state| state.sequence(|state| state.match_string("schema").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn meta_statement(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::meta_statement, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn language_statement(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::language_statement, |state| state.sequence(|state| state.match_string("language").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| self::language_inherit(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn language_inherit(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::language_inherit, |state| state.sequence(|state| state.match_string(":").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object_inherit(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::object_inherit, |state| state.sequence(|state| state.optional(|state| self::namespace(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::object, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| self::SEPARATOR(state).or_else(|state| self::pair(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| self::SEPARATOR(state).or_else(|state| self::pair(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn pair(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::pair, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| self::object_inherit(state).or_else(|state| self::list(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::list, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::value(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::value(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| self::Set(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| self::value(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_string("=").or_else(|state| state.match_string(":"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| state.rule(Rule::value, |state| self::String(state).or_else(|state| self::Special(state)).or_else(|state| self::Color(state)).or_else(|state| self::Number(state)).or_else(|state| self::namespace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Special(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::Special, |state| state.atomic(Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Color(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::Color, |state| state.atomic(Atomic, |state| state.sequence(|state| state.match_string("#").and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| state.repeat(|state| self::ASCII_HEX_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| self::SignedNumber(state).and_then(|state| state.optional(|state| self::SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SignedNumber(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(CompoundAtomic, |state| state.rule(Rule::SignedNumber, |state| state.sequence(|state| state.optional(|state| self::Sign(state)).and_then(|state| self::Decimal(state).or_else(|state| self::DecimalBad(state)).or_else(|state| self::Integer(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| self::Integer(state).and_then(|state| state.match_string(".")).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(CompoundAtomic, |state| state.rule(Rule::DecimalBad, |state| state.sequence(|state| self::Integer(state).and_then(|state| state.match_string("."))).or_else(|state| state.sequence(|state| state.match_string(".").and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(Atomic, |state| state.sequence(|state| self::ASCII_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| self::ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(Atomic, |state| state.match_string("+").or_else(|state| state.match_string("-"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.match_string("\"").and_then(|state| state.repeat(|state| self::Character(state))).and_then(|state| state.match_string("\"")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Character(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::Character, |state| state.atomic(Atomic, |state| state.sequence(|state| state.match_string("\\u").and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state)).and_then(|state| self::ASCII_HEX_DIGIT(state))).or_else(|state| state.sequence(|state| state.match_string("\\u").and_then(|state| state.match_string("{")).and_then(|state| state.sequence(|state| self::ASCII_HEX_DIGIT(state).or_else(|state| self::SPACE_SEPARATOR(state)).and_then(|state| state.repeat(|state| self::ASCII_HEX_DIGIT(state).or_else(|state| self::SPACE_SEPARATOR(state)))))).and_then(|state| state.match_string("}")))).or_else(|state| state.sequence(|state| state.match_string("\\").and_then(|state| self::ANY(state)))).or_else(|state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("\"")).and_then(|state| self::ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("\"")).and_then(|state| self::ANY(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn namespace(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::namespace, |state| state.sequence(|state| self::SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.match_string(".").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(".").and_then(|state| super::hidden::skip(state)).and_then(|state| self::SYMBOL(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(Atomic, |state| state.sequence(|state| self::XID_START(state).and_then(|state| state.repeat(|state| self::XID_CONTINUE(state).or_else(|state| state.match_string("_")).or_else(|state| state.match_string("-")))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(Atomic, |state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| self::NEWLINE(state)).and_then(|state| self::ANY(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.atomic(Atomic, |state| self::NEWLINE(state).or_else(|state| self::SPACE_SEPARATOR(state)).or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SEPARATOR(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_string(",").or_else(|state| state.match_string(";"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<ParserState<Rule>>) -> ParseResult<Box<ParserState<Rule>>> {
                    state.match_char_by(::pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::EmptyLine => rules::EmptyLine(state),
            Rule::import_statement => rules::import_statement(state),
            Rule::schema_statement => rules::schema_statement(state),
            Rule::meta_statement => rules::meta_statement(state),
            Rule::language_statement => rules::language_statement(state),
            Rule::language_inherit => rules::language_inherit(state),
            Rule::object_inherit => rules::object_inherit(state),
            Rule::object => rules::object(state),
            Rule::pair => rules::pair(state),
            Rule::list => rules::list(state),
            Rule::Set => rules::Set(state),
            Rule::value => rules::value(state),
            Rule::Special => rules::Special(state),
            Rule::Color => rules::Color(state),
            Rule::Number => rules::Number(state),
            Rule::SignedNumber => rules::SignedNumber(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Integer => rules::Integer(state),
            Rule::Sign => rules::Sign(state),
            Rule::String => rules::String(state),
            Rule::Character => rules::Character(state),
            Rule::namespace => rules::namespace(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::COMMENT => rules::COMMENT(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::SEPARATOR => rules::SEPARATOR(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
