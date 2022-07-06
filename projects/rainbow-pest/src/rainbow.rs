pub struct RainbowParser;
#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    EOI,
    program,
    statement,
    schema_statement,
    meta_statement,
    language_statement,
    inherit,
    object_inherit,
    object,
    pair,
    list,
    Set,
    value,
    Special,
    Color,
    Number,
    Decimal,
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
impl pest::Parser<Rule> for RainbowParser {
    fn parse<'i>(rule: Rule, input: &'i str) -> Result<pest::iterators::Pairs<'i, Rule>, pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    if state.atomicity() == pest::Atomicity::NonAtomic { state.sequence(|state| state.repeat(|state| super::visible::WHITESPACE(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::visible::COMMENT(state).and_then(|state| state.repeat(|state| super::visible::WHITESPACE(state))))))) } else { Ok(state) }
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.sequence(|state| SOI(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| statement(state).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| statement(state)))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| EOI(state)))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    SEPARATOR(state).or_else(|state| schema_statement(state)).or_else(|state| language_statement(state)).or_else(|state| meta_statement(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn schema_statement(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::schema_statement, |state| state.sequence(|state| state.match_string("schema").and_then(|state| super::hidden::skip(state)).and_then(|state| SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| inherit(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn meta_statement(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::meta_statement, |state| state.sequence(|state| SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn language_statement(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::language_statement, |state| state.sequence(|state| state.match_string("language").and_then(|state| super::hidden::skip(state)).and_then(|state| SYMBOL(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| state.optional(|state| inherit(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn inherit(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::inherit, |state| state.sequence(|state| state.match_string(":").and_then(|state| super::hidden::skip(state)).and_then(|state| namespace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object_inherit(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::object_inherit, |state| state.sequence(|state| state.optional(|state| namespace(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| object(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn object(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::object, |state| state.sequence(|state| state.match_string("{").and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| SEPARATOR(state).or_else(|state| pair(state)).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| SEPARATOR(state).or_else(|state| pair(state))))))))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.match_string("}"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn pair(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::pair, |state| state.sequence(|state| SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| Set(state)).and_then(|state| super::hidden::skip(state)).and_then(|state| object_inherit(state).or_else(|state| list(state)))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn list(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::list, |state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| Set(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| value(state))).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| Set(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| value(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.lookahead(false, |state| state.sequence(|state| SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| Set(state)))).and_then(|state| super::hidden::skip(state)).and_then(|state| value(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Set(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_string("=").or_else(|state| state.match_string(":"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn value(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.atomic(pest::Atomicity::NonAtomic, |state| state.rule(Rule::value, |state| String(state).or_else(|state| Special(state)).or_else(|state| Color(state)).or_else(|state| Number(state)).or_else(|state| namespace(state))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Special(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::Special, |state| state.atomic(pest::Atomicity::Atomic, |state| state.match_string("true").or_else(|state| state.match_string("false")).or_else(|state| state.match_string("null"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Color(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::Color, |state| state.atomic(pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("#").and_then(|state| ASCII_HEX_DIGIT(state)).and_then(|state| state.repeat(|state| ASCII_HEX_DIGIT(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.atomic(pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Number, |state| state.sequence(|state| state.optional(|state| Sign(state)).and_then(|state| Decimal(state).or_else(|state| Integer(state))).and_then(|state| state.optional(|state| SYMBOL(state))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.atomic(pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::Decimal, |state| state.sequence(|state| Integer(state).and_then(|state| state.match_string(".")).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| state.atomic(pest::Atomicity::Atomic, |state| state.sequence(|state| ASCII_DIGIT(state).and_then(|state| state.repeat(|state| state.sequence(|state| state.optional(|state| state.match_string("_")).and_then(|state| ASCII_DIGIT(state))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Sign(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::Sign, |state| state.atomic(pest::Atomicity::Atomic, |state| state.match_string("+").or_else(|state| state.match_string("-"))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.atomic(pest::Atomicity::CompoundAtomic, |state| state.rule(Rule::String, |state| state.sequence(|state| state.match_string("\"").and_then(|state| state.repeat(|state| Character(state))).and_then(|state| state.match_string("\"")))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Character(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::Character, |state| state.atomic(pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("\\u").and_then(|state| ASCII_HEX_DIGIT(state)).and_then(|state| ASCII_HEX_DIGIT(state)).and_then(|state| ASCII_HEX_DIGIT(state)).and_then(|state| ASCII_HEX_DIGIT(state))).or_else(|state| state.sequence(|state| state.match_string("\\u").and_then(|state| state.match_string("{")).and_then(|state| state.sequence(|state| ASCII_HEX_DIGIT(state).or_else(|state| SPACE_SEPARATOR(state)).and_then(|state| state.repeat(|state| ASCII_HEX_DIGIT(state).or_else(|state| SPACE_SEPARATOR(state)))))).and_then(|state| state.match_string("}")))).or_else(|state| state.sequence(|state| state.match_string("\\").and_then(|state| ANY(state)))).or_else(|state| state.sequence(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("\"")).and_then(|state| ANY(state))).and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| state.match_string("\"")).and_then(|state| ANY(state)))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn namespace(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::namespace, |state| state.sequence(|state| SYMBOL(state).and_then(|state| super::hidden::skip(state)).and_then(|state| state.sequence(|state| state.optional(|state| state.sequence(|state| state.match_string(".").and_then(|state| super::hidden::skip(state)).and_then(|state| SYMBOL(state))).and_then(|state| state.repeat(|state| state.sequence(|state| super::hidden::skip(state).and_then(|state| state.sequence(|state| state.match_string(".").and_then(|state| super::hidden::skip(state)).and_then(|state| SYMBOL(state))))))))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| state.atomic(pest::Atomicity::Atomic, |state| state.sequence(|state| XID_START(state).and_then(|state| state.repeat(|state| XID_CONTINUE(state).or_else(|state| state.match_string("_")).or_else(|state| state.match_string("-")))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn COMMENT(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.atomic(pest::Atomicity::Atomic, |state| state.sequence(|state| state.match_string("//").and_then(|state| state.repeat(|state| state.sequence(|state| state.lookahead(false, |state| NEWLINE(state)).and_then(|state| ANY(state)))))))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.atomic(pest::Atomicity::Atomic, |state| NEWLINE(state).or_else(|state| SPACE_SEPARATOR(state)).or_else(|state| state.match_string("\t")))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SEPARATOR(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_string(",").or_else(|state| state.match_string(";"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn NEWLINE(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r"))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_CONTINUE(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_char_by(pest::unicode::XID_CONTINUE)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn XID_START(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_char_by(pest::unicode::XID_START)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                fn SPACE_SEPARATOR(state: Box<pest::ParserState<Rule>>) -> pest::ParseResult<Box<pest::ParserState<Rule>>> {
                    state.match_char_by(pest::unicode::SPACE_SEPARATOR)
                }
            }
            pub use self::visible::*;
        }
        pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::schema_statement => rules::schema_statement(state),
            Rule::meta_statement => rules::meta_statement(state),
            Rule::language_statement => rules::language_statement(state),
            Rule::inherit => rules::inherit(state),
            Rule::object_inherit => rules::object_inherit(state),
            Rule::object => rules::object(state),
            Rule::pair => rules::pair(state),
            Rule::list => rules::list(state),
            Rule::Set => rules::Set(state),
            Rule::value => rules::value(state),
            Rule::Special => rules::Special(state),
            Rule::Color => rules::Color(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
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
