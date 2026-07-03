use bevy::ecs::{observer::On, system::ResMut};

use crate::shop::{components::Money, events::AddMoneyEvent};

pub fn on_add_money(event: On<AddMoneyEvent>, mut money: ResMut<Money>) {
    money.0 += event.0
}
