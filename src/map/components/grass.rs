use bevy::{
    ecs::observer::On,
    log::warn,
    picking::{
        Pickable,
        events::{Pointer, Press},
    },
    scene::{Scene, SceneComponent, bsn, on},
    sprite::{Anchor, Sprite},
};

use crate::map::components::Block;

#[derive(SceneComponent, Default, Clone)]
pub struct Grass;

impl Grass {
    pub fn scene() -> impl Scene {
        bsn! {
            #Grass
            Block
            Sprite {
                image: "blocks/grass.png",
            }
            Anchor::BOTTOM_CENTER
            Pickable::default()
            on(|click: On<Pointer<Press>>| warn!("test"))
        }
    }
}
