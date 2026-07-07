pub mod componnets;
mod systems;

use bevy::{
    app::{Plugin, Update},
    color::palettes::css::WHITE,
    ecs::{hierarchy::Children, observer::On, schedule::IntoScheduleConfigs, system::Commands},
    picking::events::{Pointer, Press},
    scene::{CommandsSceneExt, bsn, on},
    state::{condition::in_state, state::OnEnter},
    text::{TextColor, TextFont},
    ui::{Node, px, widget::Text},
};

use crate::{
    shop::events::AddMoneyEvent,
    states::AppState,
    ui::{componnets::MoneyTextUi, systems::update_money},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, (update_money).run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands) {
    commands.queue_spawn_scene(bsn! {
        Node {
            padding: px(20),
            right: px(0)
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
