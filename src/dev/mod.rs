use std::time::Duration;

use avian2d::debug_render::PhysicsDebugPlugin;
use bevy::{
    app::{Plugin, PostStartup},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    ecs::{query::With, system::Query},
    ui::{Node, PositionType, Val},
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(
            (
                // EguiPlugin::default(),
                // WorldInspectorPlugin::new(),
                // PhysicsDebugPlugin,
                FpsOverlayPlugin::default()
            ),
        );
    }
}
