use std::{collections::BTreeMap, str::FromStr};

use rainbow_pest::{
    ast::{ASTProgram, ASTStatement, MetaStatement, RangedObject, RangedValue, SchemaStatement},
    ParserConfig,
};

use crate::{schema::Value, RainbowError, RainbowVM, Result, Schema};

struct EvalState {
    first_schema: bool,
}

impl Default for EvalState {
    fn default() -> Self {
        Self { first_schema: true }
    }
}

impl RainbowVM {
    pub fn define_schema(&mut self, text: &str) -> Result<()> {
        let ast = ASTProgram::from_str(text)?;
        self.eval_ast(ast)
    }
    fn eval_ast(&mut self, program: ASTProgram) -> Result<()> {
        let mut out = Schema::default();
        let mut ctx = EvalState::default();
        for i in program.statements {
            match i {
                ASTStatement::Schema(node) => {
                    println!("{:#?}", node);
                    // out.eval_schema(node, &mut ctx)?;
                }
                ASTStatement::Meta(node) => {
                    println!("{:#?}", node);
                    // out.eval_meta(node, &mut ctx)?;
                }
                ASTStatement::Language(node) => {
                    println!("{:#?}", node);
                }
            }
        }
        todo!()
    }
    fn eval_schema(&mut self, ast: SchemaStatement, ctx: &mut EvalState) -> Result<()> {
        if ctx.first_schema {
            ctx.first_schema = false
        }
        else {
            return Err(RainbowError::duplicate_declaration("schema"));
        }
        todo!()
        // if let Some(v) = ast.object.get_string("theme") {
        //     self.theme = v
        // }
        // if let Some(v) = ast.object.get_string("variant") {
        //     self.variant = v
        // }
        // Ok(())
    }
    fn eval_meta(&mut self, ast: MetaStatement, ctx: &mut EvalState) -> Result<()> {
        todo!()
    }
    fn eval_object(o: RangedObject, ctx: &mut EvalState) -> Result<Self> {
        let mut out = BTreeMap::new();
        for (k, ranged) in o.inner {
            let v = match ranged {
                RangedValue::Null => Value::null(),
                RangedValue::String(v) => Value::string(v),
                RangedValue::Number(v) => Value::number(v),
                RangedValue::Boolean(v) => Value::boolean(v),
                RangedValue::Color(v) => Value::color(v),
                RangedValue::Array(v) => {
                    todo!()
                }
                RangedValue::Namespace(v) => Value::reference(v),
                RangedValue::Object(v) => todo!(),
            };
            out.insert(k, v);
        }
        return todo!();
    }
}
