mod camera;
mod dev;
mod shop;
mod ui;

use bevy::prelude::*;

use crate::{camera::CameraPlugin, dev::DevPlugin, shop::ShopPlugin, ui::UiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((DevPlugin, UiPlugin, CameraPlugin, ShopPlugin))
        .run();
}
