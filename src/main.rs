mod camera;
mod dev;
mod map;
mod shop;
mod ui;

use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::{camera::CameraPlugin, dev::DevPlugin, map::MapPlugin, shop::ShopPlugin, ui::UiPlugin};

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(ImagePlugin::default_nearest()),
        ))
        .add_plugins(DevPlugin)
        .add_plugins((UiPlugin, CameraPlugin, ShopPlugin, MapPlugin))
        .run();
}
