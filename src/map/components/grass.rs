use bevy::{
    asset::asset_value,
    ecs::{observer::On, system::Res},
    image::Image,
    log::{tracing_subscriber::reload::Handle, warn},
    math::primitives::Rectangle,
    mesh::Mesh2d,
    picking::{
        Pickable,
        events::{Pointer, Press},
    },
    scene::{Scene, SceneComponent, bsn, on},
    sprite::Anchor,
    sprite_render::{ColorMaterial, MeshMaterial2d},
};

use crate::map::{components::Block, resources::BlockAssets};

#[derive(SceneComponent, Default, Clone)]
pub struct Grass;

impl Grass {
    pub fn scene() -> impl Scene {
        bsn! {
            #Grass
            Block
            Mesh2d(asset_value(Rectangle::new(32., 32.)))
            MeshMaterial2d<ColorMaterial>(asset_value(ColorMaterial {
                texture: Some(asset_value::<&'static str, Image>("grass.png").into()),
                ..Default::default()
            }))
            Anchor::BOTTOM_CENTER
            Pickable::default()
            on(|click: On<Pointer<Press>>| warn!("test"))
        }
    }
}
