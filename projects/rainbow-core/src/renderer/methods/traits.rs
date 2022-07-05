use super::*;

impl Debug for RenderFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Debug for RenderNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("name", &self.name.join("."))
            .field("attributes", &self.attributes)
            .field("children", &self.text)
            .finish()
    }
}

impl Default for RenderFragment {
    fn default() -> Self {
        Self { inner: vec![] }
    }
}
