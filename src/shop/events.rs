use bevy::ecs::event::Event;

#[derive(Event)]
pub struct AddMoneyEvent(pub u32);
