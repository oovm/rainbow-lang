use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::ast::{ASTProgram, ASTStatement, KvPair, Object, RangedValue};
use crate::{RainbowParser, Rule};

pub use self::config::ParserConfig;

mod config;

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

type Result<T> = std::result::Result<T, Error<Rule>>;

impl ParserConfig {
    ///
    pub fn parse(&self, input: &str) -> Result<ASTProgram> {
        let parsed = RainbowParser::parse(Rule::program, &input)?;
        self.parse_program(parsed)
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> Result<ASTProgram> {
        let mut codes = vec![];
        let mut additional = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI | Rule::SEPARATOR => continue,
                Rule::schema_statement => codes.push(self.parse_schema(pair)?),
                Rule::COMMENT => additional = Some(pair.as_str().to_string()),
                _ => debug_cases!(pair),
            };
        }
        todo!()
        // AST { kind: ASTKind::program(codes), range: Default::default(), additional }
    }
    fn parse_schema(&self, pairs: Pair<Rule>) -> Result<ASTStatement> {
        // let r = self.get_position(&pairs);
        let mut symbol = String::new();
        let mut object = Object::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => symbol = pair.to_string(),
                Rule::object => object = self.parse_object(pair)?,
                _ => debug_cases!(pair),
            };
        }
        todo!()
    }
}

impl ParserConfig {
    fn parse_object(&self, pairs: Pair<Rule>) -> Result<Object> {
        let mut object = Object::new();
        for pair in pairs.into_inner() {
            let pair = self.parse_pair(pair)?;
            object.insert_pair(pair)
        }
        todo!()
    }
    fn parse_object_inherit(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        for pair in pairs.into_inner() {
            println!("{:?}", pair)
        }
        todo!()
    }
    fn parse_list(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        let mut out = vec![];
        for pair in pairs.into_inner() {
            out.push(self.parse_value(pair)?)
        }
        Ok(RangedValue::Array(out))
    }
    fn parse_pair(&self, pairs: Pair<Rule>) -> Result<KvPair> {
        let mut pairs = pairs.into_inner();
        let key = pairs.next().unwrap().to_string();
        let pair = pairs.next().unwrap();
        let value = match pair.as_rule() {
            Rule::list => self.parse_list(pair)?,
            Rule::object_inherit => self.parse_object_inherit(pair)?,
            _ => unreachable!(),
        };
        Ok(KvPair::new(key, value))
    }
    fn parse_value(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        for pair in pairs.into_inner() {
            println!("{:?}", pair)
        }
        todo!()
    }
}
