pub mod components;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::system::Commands,
    math::I8Vec3,
    scene::{CommandsSceneExt, bsn},
};

use crate::map::{
    components::{BlockPos, grass::Grass},
    systems::fix_block_position,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, fix_block_position);
    }
}

fn setup(mut commands: Commands) {
    for x in -25..25 {
        for y in -25..25 {
            commands.spawn_scene(bsn! {
                @Grass
                BlockPos(I8Vec3 { x: x, y: y, z: 0 })
            });
        }
    }
}
