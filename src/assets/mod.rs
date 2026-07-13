use bevy::app::Plugin;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::states::AppState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame),
        );
    }
}
