// use crate::utils::parse_format;
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtendFormat {
    TEXT,
    JSON,
    HJSON,
    TOML,
    ARC,
    YAML,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtendStatement {
    format: ExtendFormat,
    path: Option<PathBuf>,
}

impl ExtendStatement {
    pub fn new(_format: String, _path: String, _this: Option<PathBuf>) -> Box<Self> {
        unimplemented!()
        // let o = this.and_then(|e| e.parent()).map(|&e| e.join(PathBuf::from(path)));
        // let out = Self {
        //     format: parse_format(format),
        //     path: o,
        // };
        // return box out;
    }
}
