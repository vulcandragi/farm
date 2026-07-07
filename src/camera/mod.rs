use bevy::{
    app::Plugin,
    camera::{Camera2d, ClearColor, OrthographicProjection, Projection, ScalingMode},
    color::Color,
    ecs::system::Commands,
    state::state::OnEnter,
};

use crate::states::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .insert_resource(ClearColor(Color::srgb_u8(153, 217, 222)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 1280.,
                max_height: 720.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
