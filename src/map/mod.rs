pub mod components;

use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    math::Vec3,
    scene::{CommandsSceneExt, bsn},
};

use crate::map::components::{Block, BlockType};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        @Block {
            @pos: Vec3::ZERO,
            @block_type: BlockType::Grass
        }
    });
}
