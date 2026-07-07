#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var sprite_texture: texture_2d<f32>;
@group(2) @binding(1) var sprite_sampler: sampler;

struct OutlineSettings {
    color: vec4<f32>,
    thickness: f32,
};

@group(2) @binding(2) var<uniform> settings: OutlineSettings;

@fragment
fn fragment(mesh: VertextOutput) -> @location vec4<f32> {
let uv = mesh.uv;
    let tex_color = textureSample(sprite_texture, sprite_sampler, uv);

    if tex_color.a > 0.1 {
        return tex_color;
    }

    let t = settings.thickness;
    var outline_alpha: f32 = 0.0;

    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(t, 0.0)).a;
    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(-t, 0.0)).a;
    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(0.0, t)).a;
    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(0.0, -t)).a;

    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(t, t)).a;
    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(-t, t)).a;
    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(t, -t)).a;
    outline_alpha += textureSample(sprite_texture, sprite_sampler, uv + vec2<f32>(-t, -t)).a;

    if outline_alpha > 0.0 {
        return settings.color;
    }

    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}
