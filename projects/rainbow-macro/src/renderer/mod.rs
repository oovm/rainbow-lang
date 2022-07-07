use std::lazy::SyncLazy;

use rainbow_core::RainbowVM;
use rainbow_render::backend::HtmlInlineRenderer;

pub static VM: SyncLazy<RainbowVM> = SyncLazy::new(|| RainbowVM::builtin());

fn render<'vm>() -> HtmlInlineRenderer<'vm> {
    //  HtmlInlineRenderer::new(&VM).render()
    todo!()
}
