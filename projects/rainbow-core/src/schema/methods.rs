use super::*;

impl Default for Schema {
    fn default() -> Self {
        Self {
            schema: "".to_string(),
            theme: "".to_string(),
            variant: "".to_string(),
            custom: Default::default(),
            language: Default::default(),
        }
    }
}

impl Schema {}
