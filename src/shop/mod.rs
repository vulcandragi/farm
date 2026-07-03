pub mod components;
pub mod events;
mod systems;

use bevy::app::Plugin;

use crate::shop::{components::Money, systems::on_add_money};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(Money(0)).add_observer(on_add_money);
    }
}
