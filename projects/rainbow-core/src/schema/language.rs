use super::*;

impl Language {
    pub fn get(&self, token: &str) -> Option<&Value> {
        self.tokens.get(token)
    }
}
