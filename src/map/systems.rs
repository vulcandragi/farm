use bevy::{
    ecs::{
        query::{Changed, With}, system::Query,
    }, math::Vec3, transform::components::Transform,
};

use crate::map::components::{Block, BlockPos};

pub fn fix_block_position(
    mut query: Query<(&mut Transform, &BlockPos), (With<Block>, Changed<BlockPos>)>,
) {
    for (mut transform, BlockPos(position)) in query.iter_mut() {
        transform.translation = Vec3::new(
            ((position.x - position.y) * 16) as f32,
            ((position.x + position.y) * 8) as f32,
            -(position.x + position.y) as f32,
        )
    }
}
