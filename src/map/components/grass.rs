use bevy::{
    asset::{Assets, asset_value},
    color::Color,
    ecs::{observer::On, system::Commands, template::template},
    math::primitives::Rectangle,
    mesh::Mesh2d,
    picking::{
        Pickable,
        events::{Out, Over, Pointer},
    },
    scene::{Scene, SceneComponent, bsn, on},
    sprite::Anchor,
    sprite_render::{ColorMaterial, MeshMaterial2d},
};

use crate::{
    effects::outline::componets::SpriteOutline,
    map::{components::Block, resources::BlockAssets},
};

#[derive(SceneComponent, Default, Clone)]
pub struct Grass;

impl Grass {
    pub fn scene() -> impl Scene {
        bsn! {
            #Grass
            Block
            Mesh2d(asset_value(Rectangle::new(32., 32.)))
            Anchor::BOTTOM_CENTER
            Pickable::default()
            on(Grass::on_hover_enter)
            on(Grass::on_hover_left)
            template(|context| {
                let texture = context.resource::<BlockAssets>().grass.clone();
                let mut meterial = context.resource_mut::<Assets<ColorMaterial>>();

                Ok(MeshMaterial2d::<ColorMaterial>(meterial.add(ColorMaterial {
                    texture: Some(texture),
                    ..Default::default()
                })))
            })
        }
    }

    fn on_hover_enter(event: On<Pointer<Over>>, mut commands: Commands) {
        commands.entity(event.entity).insert(SpriteOutline {
            color: Color::WHITE,
            thickness: 1,
        });
    }

    fn on_hover_left(event: On<Pointer<Out>>, mut commands: Commands) {
        commands.entity(event.entity).remove::<SpriteOutline>();
    }
}
