use bevy::{asset::Handle, ecs::component::Component, sprite_render::ColorMaterial};

#[derive(Component)]
pub struct OriginalMaterial(pub Handle<ColorMaterial>);