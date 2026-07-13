pub mod componets;
mod materials;
mod systems;

use bevy::{app::Plugin, sprite_render::Material2dPlugin};

use crate::effects::outline::{
    materials::OutlineMaterial,
    systems::{on_add_outile, on_remove_outile},
};

pub struct OutlinePlugin;

impl Plugin for OutlinePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(Material2dPlugin::<OutlineMaterial>::default())
            .add_observer(on_add_outile)
            .add_observer(on_remove_outile);
    }
}
