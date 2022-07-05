use super::*;

impl Debug for RenderNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("name", &self.name.join("."))
            .field("attributes", &self.attributes)
            .field("children", &self.text)
            .finish()
    }
}
