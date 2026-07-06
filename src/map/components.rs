use bevy::{
    math::Vec3,
    scene::{Scene, SceneComponent, bsn},
    sprite::Sprite,
};

#[derive(Default, Clone)]
pub enum BlockType {
    #[default]
    None,
    Grass,
}

impl BlockType {
    pub fn to_sprite(&self) -> String {
        match self {
            BlockType::Grass => "blocks/grass.png".into(),
            _ => "".into(),
        }
    }
}

#[derive(SceneComponent, Default, Clone)]
#[scene(BlockProps)]
pub struct Block;

#[derive(Default)]
pub struct BlockProps {
    pub pos: Vec3,
    pub block_type: BlockType,
}

impl Block {
    pub fn scene(props: BlockProps) -> impl Scene {
        let sprite = props.block_type.to_sprite();

        bsn! {
            #Block
            Sprite {
                image: sprite
            }
        }
    }
}
