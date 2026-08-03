use avian2d::collision::collider::Collider;
use bevy::{
    ecs::{
        component::Component,
        lifecycle::Add,
        observer::On,
        query::With,
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    picking::{
        Pickable,
        events::{Out, Over, Pointer},
    },
    sprite::Sprite,
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
                Sprite {
                    image: texture.grass.clone(),
                    ..Default::default()
                },
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

    fn on_hover_enter(event: On<Pointer<Over>>, mut commands: Commands, texture: Res<BlockAssets>) {
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
