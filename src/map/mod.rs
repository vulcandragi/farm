pub mod components;
mod resources;
mod systems;

use bevy::{
    app::{Plugin, Update},
    ecs::{schedule::IntoScheduleConfigs, system::Commands},
    math::IVec3,
    state::{condition::in_state, state::OnEnter},
};
use bevy_asset_loader::loading_state::{
    LoadingStateAppExt,
    config::{ConfigureLoadingState, LoadingStateConfig},
};

use crate::{
    map::{
        components::{BlockPos, grass::Grass},
        resources::BlockAssets,
        systems::fix_block_position,
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
        .add_systems(
            Update,
            (fix_block_position).run_if(in_state(AppState::InGame)),
        )
        .add_observer(Grass::on_spawn);
    }
}

fn setup(mut commands: Commands) {
    for x in -25..25 {
        for y in -25..25 {
            commands.spawn((Grass, BlockPos(IVec3 { x, y, z: 0 })));
        }
    }
}
