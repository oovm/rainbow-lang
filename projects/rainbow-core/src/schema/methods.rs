use super::*;

impl Default for Schema {
    fn default() -> Self {
        Self {
            theme: "anonymous".to_string(),
            variant: "light".to_string(),
            default: Default::default(),
            custom: Default::default(),
            language: Default::default(),
        }
    }
}

impl Schema {}
