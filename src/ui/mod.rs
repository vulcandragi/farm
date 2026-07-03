pub mod componnets;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    color::palettes::css::WHITE,
    ecs::{hierarchy::Children, observer::On, system::Commands},
    picking::events::{Pointer, Press},
    scene::{CommandsSceneExt, bsn, on},
    text::{TextColor, TextFont},
    ui::{Node, percent, px, widget::Text},
};

use crate::{
    shop::events::AddMoneyEvent,
    ui::{componnets::MoneyTextUi, systems::update_money},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_money);
    }
}

fn setup(mut commands: Commands) {
    commands.queue_spawn_scene(bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            padding: px(20),
        }
        Children [
            MoneyTextUi
            Text("Money: 0")
            TextColor(WHITE)
            TextFont { font_size: px(40) }
            on(add_money)
        ]
    });
}

fn add_money(_: On<Pointer<Press>>, mut commands: Commands) {
    commands.trigger(AddMoneyEvent(10));
}
