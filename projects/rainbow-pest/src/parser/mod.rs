use hex_color::HexColor;
use pest::error::Error;
use pest::error::ErrorVariant::CustomError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

use crate::ast::{ASTProgram, ASTStatement, KvPair, MetaStatement, Object, RangedValue, SchemaStatement};
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
        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI | Rule::SEPARATOR => continue,
                Rule::schema_statement => codes.push(ASTStatement::Schema(self.parse_schema(pair)?)),
                Rule::meta_statement => codes.push(ASTStatement::Meta(self.parse_meta(pair)?)),
                _ => debug_cases!(pair),
            };
        }
        Ok(ASTProgram { statements: codes })
    }
    fn parse_schema(&self, pairs: Pair<Rule>) -> Result<SchemaStatement> {
        // let r = self.get_position(&pairs);
        let mut symbol = String::new();
        let mut object = Object::new();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => symbol = pair.as_str().to_string(),
                Rule::object => object = self.parse_object(pair)?,
                _ => debug_cases!(pair),
            };
        }
        Ok(SchemaStatement { schema: symbol, object })
    }
    fn parse_meta(&self, pairs: Pair<Rule>) -> Result<MetaStatement> {
        let mut pairs = pairs.into_inner();
        let meta = pairs.next().unwrap().as_str().to_string();
        let object = self.parse_object(pairs.next().unwrap())?;
        Ok(MetaStatement { meta, object })
    }
}

impl ParserConfig {
    fn parse_object(&self, pairs: Pair<Rule>) -> Result<Object> {
        let mut object = Object::new();
        for pair in pairs.into_inner() {
            let pair = self.parse_pair(pair)?;
            object.insert_pair(pair)
        }
        return Ok(object);
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
        let key = pairs.next().unwrap().as_str();
        let pair = pairs.next().unwrap();
        let value = match pair.as_rule() {
            Rule::list => self.parse_list(pair)?,
            Rule::object_inherit => self.parse_object_inherit(pair)?,
            _ => unreachable!(),
        };
        Ok(KvPair::new(key.to_string(), value))
    }
    fn parse_namespace(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        let mut out = vec![];
        for pair in pairs.into_inner() {
            out.push(pair.as_str().to_string())
        }
        Ok(RangedValue::Namespace(out))
    }
    fn parse_value(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        let pair = pairs.into_inner().nth(0).unwrap();
        let value = match pair.as_rule() {
            Rule::String => self.parse_string(pair)?,
            Rule::namespace => self.parse_namespace(pair)?,
            Rule::Color => self.parse_color(pair)?,
            _ => debug_cases!(pair),
        };
        Ok(value)
    }
    fn parse_string(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        let mut out = String::new();
        for pair in pairs.into_inner() {
            match pair.as_str() {
                s if s.starts_with('\\') => {
                    panic!("{}", s)
                }
                s => out.push_str(&s),
            }
        }
        Ok(RangedValue::String(out))
    }
    fn parse_color(&self, pairs: Pair<Rule>) -> Result<RangedValue> {
        match pairs.as_str().parse::<HexColor>() {
            Ok(o) => Ok(RangedValue::Color(o)),
            Err(e) => Err(Error::new_from_span(CustomError { message: e.to_string() }, pairs.as_span())),
        }
    }
}
