#[cfg(feature = "html")]
pub use self::renderer::parse_rainbow_html;
pub use self::{
    errors::{ErrorKind, RainbowError, Result},
    renderer::{RainbowRenderer, RenderNode},
    schema::Schema,
    vm::RainbowVM,
};

mod errors;
mod renderer;
mod schema;
mod vm;
