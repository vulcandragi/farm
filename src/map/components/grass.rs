use avian2d::collision::collider::Collider;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        lifecycle::Add,
        observer::On,
        query::{self, With},
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3, primitives::Rectangle},
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
        query: Query<&BlockPos, With<Grass>>,
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
        let position = query.get(event.entity).unwrap().0;

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
                Transform {
                    translation: Vec3::new(
                        ((position.x - position.y) * 16) as f32,
                        ((position.x + position.y) * 8) as f32,
                        -(position.x + position.y) as f32,
                    ),
                    ..Default::default()
                },
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
            Transform::from_xyz(0., 0., 2.),
        ));
    }

    fn on_hover_left(event: On<Pointer<Out>>, mut commands: Commands) {
        commands.entity(event.entity).despawn_children();
    }
}
