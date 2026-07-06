pub mod grass;

use bevy::{ecs::component::Component, math::IVec3};

#[derive(Component, Default, Clone)]
#[require(BlockPos)]
pub struct Block;

#[derive(Component, Default, Clone)]
pub struct BlockPos(pub IVec3);
