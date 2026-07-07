pub mod outline;

use bevy::app::Plugin;

use crate::effects::outline::OutlinePlugin;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(OutlinePlugin);
    }
}
