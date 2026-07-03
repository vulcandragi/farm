use bevy::{
    ecs::{
        query::{Changed, With},
        system::Query,
    },
    ui::widget::Text,
};

use crate::{shop::components::Money, ui::componnets::MoneyTextUi};

pub fn update_money(
    query_money: Query<&Money, Changed<Money>>,
    mut query_text: Query<&mut Text, With<MoneyTextUi>>,
) {
    let Ok(money) = query_money.single() else {
        return;
    };

    let Ok(mut text) = query_text.single_mut() else {
        return;
    };

    text.0 = format!("Money: {0}", money.0)
}
