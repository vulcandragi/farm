mod assets;
mod camera;
mod dev;
mod effects;
mod map;
mod shop;
mod states;
mod ui;

use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::{
    assets::AssetsPlugin, camera::CameraPlugin, dev::DevPlugin, effects::EffectsPlugin,
    map::MapPlugin, shop::ShopPlugin, states::AppState, ui::UiPlugin,
};

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MeshPickingPlugin,
        ))
        .init_state::<AppState>()
        .add_plugins(DevPlugin)
        .add_plugins((
            AssetsPlugin,
            MapPlugin,
            UiPlugin,
            CameraPlugin,
            ShopPlugin,
            EffectsPlugin,
        ))
        .run();
}
