use super::*;
use crate::schema::Language;

impl RainbowVM {
    pub fn try_reference(&self, key: &[String]) -> Option<&Value> {
        match key {
            [head, rest @ ..] => self.schemas.get(head)?.try_reference(rest),
            _ => None,
        }
    }
}

impl Schema {
    pub fn try_reference(&self, key: &[String]) -> Option<&Value> {
        match key {
            [head, rest @ ..] => self.language.get(head)?.try_reference(rest),
            _ => None,
        }
    }
}

impl Language {
    pub fn try_reference(&self, key: &[String]) -> Option<&Value> {
        match key {
            [head, rest @ ..] => self.get(head)?.try_reference(rest),
            _ => None,
        }
    }
}

impl Value {
    pub fn try_reference(&self, key: &[String]) -> Option<&Value> {
        match key {
            [head, rest @ ..] => self.get(head)?.try_reference(rest),
            _ => None,
        }
    }
}
