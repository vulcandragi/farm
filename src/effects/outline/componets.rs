use bevy::{color::Color, ecs::component::Component};

#[derive(Component)]
pub struct SpriteOutline {
    pub color: Color,
    pub thickness: f32,
}
