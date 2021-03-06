use oxygengine_composite_renderer::{component::CompositeTransform, math::Vec2};
use oxygengine_core::{
    app::AppBuilder,
    ecs::{Join, ReadStorage, System, Write, WriteStorage},
    hierarchy::Parent,
};
use oxygengine_physics_2d::{component::RigidBody2d, resource::Physics2dWorld};

pub mod prelude {
    pub use crate::*;
}

pub fn bundle_installer<'a, 'b>(builder: &mut AppBuilder<'a, 'b>, _: ()) {
    builder.install_system(
        ApplyPhysics2dToCompositeTransformSystem,
        "apply-physics-2d-to-composite-renderer",
        &[],
    );
}

#[derive(Debug, Default)]
pub struct ApplyPhysics2dToCompositeTransformSystem;

impl<'s> System<'s> for ApplyPhysics2dToCompositeTransformSystem {
    type SystemData = (
        Option<Write<'s, Physics2dWorld>>,
        ReadStorage<'s, RigidBody2d>,
        WriteStorage<'s, CompositeTransform>,
        ReadStorage<'s, Parent>,
    );

    fn run(&mut self, (world, bodies, mut transforms, parents): Self::SystemData) {
        if world.is_none() {
            return;
        }

        let world: &mut Physics2dWorld = &mut world.unwrap();

        for (body, transform, _) in (&bodies, &mut transforms, !&parents).join() {
            if let Some(handle) = body.handle() {
                if let Some(body) = world.body(handle) {
                    let isometry = body.position();
                    let p = isometry.translation;
                    let r = isometry.rotation.angle();
                    let s = transform.get_scale();
                    transform.apply(Vec2::new(p.x as f32, p.y as f32), r as f32, s);
                }
            }
        }
    }
}
