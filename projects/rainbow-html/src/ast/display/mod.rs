use crate::{ast::ASTKind, ASTNode};
use std::fmt::{self, Debug, Display, Formatter};

impl Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ASTNode { kind, range, additional } => {
                let mut builder = f.debug_struct("AST");
                builder.field("kind", kind);
                builder.field("range", range);
                if let Some(s) = additional {
                    builder.field("additional", s);
                }
                builder.finish()
            }
        }
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ASTKind::Namespace(names) => {
                for (index, item) in names.iter().enumerate() {
                    match &item.kind {
                        ASTKind::String(s) => write!(f, "{}", s.value)?,
                        ASTKind::Integer(n) => write!(f, "{}", n.value)?,
                        _ => return Ok(()),
                    }
                    if index + 1 != names.len() {
                        write!(f, ".")?
                    }
                }
                Ok(())
            }
            _ => Debug::fmt(self, f),
        }
    }
}
