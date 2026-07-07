use bevy::{
    asset::{Asset, Handle},
    color::LinearRgba,
    image::Image,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::Material2d,
};

#[derive(Clone, Default, ShaderType)]
pub struct OutlineSettings {
    pub color: LinearRgba,
    pub thickness: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct OutlineMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub settings: OutlineSettings,
}

impl Material2d for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline.wgsl".into()
    }
}
