mod assets;
mod camera;
mod dev;
mod effects;
mod map;
mod shop;
mod states;
mod ui;

use bevy::prelude::*;

use crate::{
    assets::AssetsPlugin, camera::CameraPlugin, dev::DevPlugin, effects::EffectsPlugin,
    map::MapPlugin, shop::ShopPlugin, states::AppState, ui::UiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<AppState>()
        .add_plugins(DevPlugin)
        .add_plugins((
            UiPlugin,
            CameraPlugin,
            ShopPlugin,
            MapPlugin,
            EffectsPlugin,
            AssetsPlugin,
        ))
        .run();
}
