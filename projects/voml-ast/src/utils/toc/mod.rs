use crate::{ast::ASTKind, Range, ASTNode};
use lsp_types::{CodeLens, Command, DocumentSymbol, SymbolKind};

#[derive(Debug)]
pub struct TOC {
    level: usize,
    name: String,
    kind: SymbolKind,
    range: Range,
    children: Vec<TOC>,
}

impl Default for TOC {
    fn default() -> Self {
        Self { level: 0, name: String::from("ROOT"), kind: SymbolKind::File, range: Default::default(), children: vec![] }
    }
}

// impl From<TOC> for DocumentSymbol {
//     fn from(toc: TOC) -> Self {
//         let details = format!("H{}", toc.children.len());
//         let children = match toc.children.len() {
//             0 => None,
//             _ => Some(toc.children.into_iter().map(From::from).collect()),
//         };
//         #[allow(deprecated)]
//         DocumentSymbol {
//             name: toc.name,
//             detail: Some(details),
//             kind: toc.kind,
//             deprecated: None,
//             range: toc.range,
//             selection_range: toc.range,
//             children,
//         }
//     }
// }

impl ASTNode {
    /// Table of contents
    pub fn toc(&self, max_depth: usize) -> TOC {
        let mut root = TOC::default();
        match &self.kind {
            ASTKind::Program(terms) | ASTKind::Sequence(terms) => {
                for term in terms {
                    match &term.kind {
                        ASTKind::DictScope(level, children) => {
                            if 1 + level > max_depth {
                                continue;
                            }
                            let parent = root.last_at_level(1 + level - 1);
                            let new = TOC { level: 1 + level, kind: SymbolKind::Class, name: children.to_string(), range: term.range, children: vec![] };
                            parent.children.push(new);
                        }
                        ASTKind::Scope(level, children) => {
                            if 1 + level > max_depth {
                                continue;
                            }
                            let parent = root.last_at_level(1 + level - 1);
                            let new = TOC { level: 1 + level, kind: SymbolKind::Variable, name: children.to_string(), range: term.range, children: vec![] };
                            parent.children.push(new);
                        }
                        _ => (),
                    }
                }
            }
            _ => {}
        };
        return root;
    }
}

impl TOC {
    fn last_at_level(&mut self, depth: usize) -> &mut TOC {
        if depth == 0 || self.children.len() == 0 { self } else { self.children.last_mut().unwrap().last_at_level(depth - 1) }
    }
    pub fn build_document(&self) -> DocumentSymbol {
        // TODO: detail [+x items]
        // let details = format!("H{}", self.children.len());
        let children = match self.children.len() {
            0 => None,
            _ => Some(self.children.iter().map(|e| e.build_document()).collect()),
        };
        #[allow(deprecated)]
        DocumentSymbol { name: self.name.to_owned(), detail: None, kind: self.kind, deprecated: None, range: self.range, selection_range: self.range, children }
    }
    fn push_code_lens(&self, heads: Vec<String>, lens: &mut Vec<CodeLens>) {
        match self.kind {
            SymbolKind::Class | SymbolKind::Variable => {
                let heads: Vec<String> = vec![heads, vec![self.name.to_owned()]].concat();
                let new = CodeLens { range: self.range, command: Some(Command { title: heads.join("."), command: "arc.extract.key".to_string(), arguments: None }), data: Some("code_lens".into()) };
                lens.push(new);
                for item in &self.children {
                    item.push_code_lens(heads.clone(), lens)
                }
            }
            _ => (),
        }
    }

    pub fn build_code_lens(&self) -> Vec<CodeLens> {
        // assert self if root!
        let lens = &mut vec![];
        for item in &self.children {
            item.push_code_lens(vec![], lens)
        }
        return lens.to_owned();
    }
}

// pub fn join_ast_list(list: &AST) -> String {
//     let mut out = String::new();
//     match &list.kind {
//         ASTKind::Namespace(name) => {
//             for i in name {
//                 out.push_str(&format!("{:?}", i.kind))
//             }
//         }
//         _ => ()
//     }
//     return out;
// }
