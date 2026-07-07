pub mod componets;
mod materials;

use bevy::{app::Plugin, sprite_render::Material2dPlugin};

use crate::effects::outline::materials::OutlineMaterial;

pub struct OutlinePlugin;

impl Plugin for OutlinePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(Material2dPlugin::<OutlineMaterial>::default());
    }
}
