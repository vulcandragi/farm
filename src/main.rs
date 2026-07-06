mod camera;
mod dev;
mod map;
mod shop;
mod ui;

use bevy::prelude::*;

use crate::{camera::CameraPlugin, dev::DevPlugin, map::MapPlugin, shop::ShopPlugin, ui::UiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DevPlugin)
        .add_plugins((UiPlugin, CameraPlugin, ShopPlugin, MapPlugin))
        .run();
}
