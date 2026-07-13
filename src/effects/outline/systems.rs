use bevy::{
    asset::Assets,
    ecs::{
        lifecycle::{Add, Remove},
        observer::On,
        system::{Commands, Query, Res, ResMut},
    },
    sprite_render::{ColorMaterial, MeshMaterial2d},
};

use crate::effects::{
    components::OriginalMaterial,
    outline::{
        componets::SpriteOutline,
        materials::{OutlineMaterial, OutlineSettings},
    },
};

pub fn on_add_outile(
    event: On<Add, SpriteOutline>,
    mut commands: Commands,
    query: Query<(&SpriteOutline, &MeshMaterial2d<ColorMaterial>)>,
    color_materials: Res<Assets<ColorMaterial>>,
    mut outile_materials: ResMut<Assets<OutlineMaterial>>,
) {
    let Ok((config, material)) = query.get(event.entity) else {
        return;
    };

    let Some(color_material) = color_materials.get(&material.0) else {
        return;
    };

    let Some(texture) = &color_material.texture else {
        return;
    };

    let new_material = outile_materials.add(OutlineMaterial {
        texture: texture.clone(),
        settings: OutlineSettings {
            color: config.color.to_linear(),
            thickness: config.thickness,
        },
    });

    commands
        .entity(event.entity)
        .insert(MeshMaterial2d::<OutlineMaterial>(new_material))
        .insert(OriginalMaterial(material.0.clone()))
        .remove::<MeshMaterial2d<ColorMaterial>>();
}

pub fn on_remove_outile(
    event: On<Remove, SpriteOutline>,
    mut commands: Commands,
    query: Query<&OriginalMaterial>,
) {
    let Ok(original) = query.get(event.entity) else {
        return;
    };

    commands
        .entity(event.entity)
        .insert(MeshMaterial2d::<ColorMaterial>(original.0.clone()))
        .remove::<MeshMaterial2d<OutlineMaterial>>()
        .remove::<OriginalMaterial>();
}
