extern crate serde;
extern crate shrev;
extern crate specs;
extern crate uuid;

pub mod app;
pub mod assets;
pub mod error;
pub mod id;
#[cfg(test)]
mod tests;

pub mod ecs {
    pub use specs::*;
}
pub mod events {
    pub use shrev::*;
}

pub mod prelude {
    pub use crate::app::*;
    pub use crate::assets::*;
    pub use crate::ecs::*;
    pub use crate::events::*;
    pub use crate::id::*;
}
