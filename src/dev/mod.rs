use bevy::app::Plugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
    }
}
