mod assets;
mod camera;
mod dev;
pub mod effects;
mod map;
mod shop;
mod states;
mod ui;

use avian2d::{PhysicsPlugins, picking::PhysicsPickingPlugin};
use bevy::{prelude::*, window::PresentMode};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::{
    assets::AssetsPlugin, camera::CameraPlugin, dev::DevPlugin, map::MapPlugin, shop::ShopPlugin,
    states::AppState, ui::UiPlugin,
};

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            PhysicsPlugins::default(),
            PhysicsPickingPlugin,
        ))
        .init_state::<AppState>()
        .add_plugins(DevPlugin)
        .add_plugins((AssetsPlugin, MapPlugin, UiPlugin, CameraPlugin, ShopPlugin))
        .run();
}
