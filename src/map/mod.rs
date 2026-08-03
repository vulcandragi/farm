pub mod components;
mod resources;

use bevy::{app::Plugin, ecs::system::Commands, math::IVec3, state::state::OnEnter};
use bevy_asset_loader::loading_state::{
    LoadingStateAppExt,
    config::{ConfigureLoadingState, LoadingStateConfig},
};

use crate::{
    map::{
        components::{BlockPos, grass::Grass},
        resources::BlockAssets,
    },
    states::AppState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading).load_collection::<BlockAssets>(),
        )
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_observer(Grass::on_spawn);
    }
}

fn setup(mut commands: Commands) {
    let size = 25;

    for x in -size..size {
        for y in -size..size {
            commands.spawn((Grass, BlockPos(IVec3 { x, y, z: 0 })));
        }
    }
}
