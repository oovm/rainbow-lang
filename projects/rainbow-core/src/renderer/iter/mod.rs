use super::*;
use std::slice::Iter;

impl<'i> IntoIterator for &'i RenderFragment {
    type Item = &'i RenderNode;
    type IntoIter = Iter<'i, RenderNode>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
