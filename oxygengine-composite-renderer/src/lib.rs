extern crate oxygengine_core as core;

pub mod component;
pub mod composite_renderer;
pub mod math;
pub mod system;

pub mod prelude {
    pub use crate::bundle_installer as composite_renderer_bundle_installer;
    pub use crate::component::*;
    pub use crate::composite_renderer::*;
    pub use crate::system::*;
}

use crate::{
    component::{CompositeRenderDepth, CompositeRenderable, CompositeTransform},
    composite_renderer::CompositeRenderer,
    system::CompositeRendererSystem,
};
use core::app::AppBuilder;

pub fn bundle_installer<'a, 'b, CR: 'static>(builder: &mut AppBuilder<'a, 'b>, data: CR)
where
    CR: CompositeRenderer + Send + Sync,
{
    builder.install_resource(data);
    builder.install_component::<CompositeRenderable>();
    builder.install_component::<CompositeTransform>();
    builder.install_component::<CompositeRenderDepth>();
    builder.install_thread_local_system(CompositeRendererSystem::<CR>::default());
}
