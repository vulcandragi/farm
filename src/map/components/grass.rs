use avian2d::collision::collider::Collider;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        lifecycle::Add,
        observer::On,
        query,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    picking::{
        Pickable,
        events::{Out, Over, Pointer},
    },
    sprite::{Anchor, Sprite},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    effects::componnets::Outline,
    map::{
        components::{Block, BlockPos},
        resources::BlockAssets,
    },
};

#[derive(Component)]
pub struct Grass;

impl Grass {
    pub fn on_spawn(
        event: On<Add, (Grass, BlockPos)>,
        mut commands: Commands,
        mut meshs: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        texture: Res<BlockAssets>,
    ) {
        let collider = Collider::convex_hull(vec![
            Vec2::new(-16., 0.),
            Vec2::new(0., -8.),
            Vec2::new(16., 0.),
            Vec2::new(0., 8.),
        ])
        .unwrap();

        commands
            .entity(event.entity)
            .insert((
                Block,
                Mesh2d(meshs.add(Rectangle::new(32., 32.))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    texture: Some(texture.grass.clone()),
                    ..Default::default()
                })),
                Anchor::BOTTOM_CENTER,
                Pickable::default(),
                collider,
            ))
            .observe(Self::on_hover_enter)
            .observe(Self::on_hover_left);
    }

    fn on_hover_enter(
        event: On<Pointer<Over>>,
        query: Query<&Transform>,
        mut commands: Commands,
        texture: Res<BlockAssets>,
    ) {
        let transform = query.get(event.entity).unwrap();

        commands.entity(event.entity).with_child((
            Outline,
            Sprite {
                image: texture.block_outilne.clone(),
                ..Default::default()
            },
            Transform::from_xyz(0., 0., transform.translation.z + 1.),
        ));
    }

    fn on_hover_left(event: On<Pointer<Out>>, mut commands: Commands) {
        commands.entity(event.entity).despawn_children();
    }
}
