use std::{collections::HashMap, str::FromStr};

use rainbow_pest::{
    ast::{ASTProgram, ASTStatement, MetaStatement, RangedObject, RangedValue, SchemaStatement},
    Error, ParserConfig, Rule,
    Rule::Set,
};

use crate::{schema::Value, RainbowError};

use super::Schema;

type Result<T> = std::result::Result<T, RainbowError>;

impl From<Error<Rule>> for RainbowError {
    fn from(e: Error<Rule>) -> Self {
        todo!()
    }
}

impl FromStr for Schema {
    type Err = RainbowError;

    fn from_str(s: &str) -> Result<Self> {
        let parser = ParserConfig::default();
        let ast = parser.parse(s)?;
        Schema::try_from(ast)
    }
}

impl TryFrom<ASTProgram> for Schema {
    type Error = RainbowError;

    fn try_from(program: ASTProgram) -> std::result::Result<Self, Self::Error> {
        let mut out = Schema::default();
        let mut ctx = SchemaContext::default();
        for i in program.statements {
            match i {
                ASTStatement::Import(node) => {
                    println!("{:#?}", node);
                    todo!()
                }
                ASTStatement::Schema(node) => {
                    out.eval_schema(node, &mut ctx)?;
                }
                ASTStatement::Meta(node) => {
                    out.eval_meta(node, &mut ctx)?;
                }
                ASTStatement::Global(node) => {
                    println!("{:#?}", node);
                    todo!()
                }
            }
        }
        todo!()
    }
}

impl Default for SchemaContext {
    fn default() -> Self {
        Self { first_schema: true, first_meta: true }
    }
}

struct SchemaContext {
    first_schema: bool,
    first_meta: bool,
}

impl Schema {
    fn eval_schema(&mut self, ast: SchemaStatement, ctx: &mut SchemaContext) -> Result<()> {
        if ctx.first_schema {
            ctx.first_schema = false
        }
        else {
            return Err(RainbowError::duplicate_declaration("schema"));
        }
        self.schema = ast.schema;
        if let Some(v) = ast.object.get_string("theme") {
            self.theme = v
        }
        if let Some(v) = ast.object.get_string("variant") {
            self.variant = v
        }
        Ok(())
    }
    fn eval_meta(&mut self, ast: MetaStatement, ctx: &mut SchemaContext) -> Result<()> {
        if ctx.first_meta {
            ctx.first_meta = false
        }
        else {
            return Err(RainbowError::duplicate_declaration("meta"));
        }
        println!("{:#?}", ast);
        todo!();
        Ok(())
    }
}

impl From<RangedObject> for Value {
    fn from(o: RangedObject) -> Self {
        let mut out = HashMap::new();
        for (k, ranged) in o.inner {
            let v = match ranged {
                RangedValue::Null => Value::Null,
                RangedValue::String(v) => {
                    todo!()
                }
                RangedValue::Number(v) => {
                    todo!()
                }
                RangedValue::Boolean(v) => {
                    todo!()
                }
                RangedValue::Color(v) => {
                    todo!()
                }
                RangedValue::Array(v) => {
                    todo!()
                }
                RangedValue::Namespace(v) => {
                    todo!()
                }
                RangedValue::Object(v) => {
                    todo!()
                }
            };
            out.insert(k, v);
        }
        return Value::Object(out);
    }
}
