use bevy::{asset::Handle, ecs::resource::Resource, image::Image};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(Resource, AssetCollection)]
pub struct BlockAssets {
    #[asset(path = "blocks/grass.png")]
    grass: Handle<Image>,
}
