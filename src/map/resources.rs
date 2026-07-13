use bevy::{asset::Handle, ecs::resource::Resource, image::Image};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct BlockAssets {
    #[asset(path = "blocks/grass.png")]
    pub grass: Handle<Image>,
}
