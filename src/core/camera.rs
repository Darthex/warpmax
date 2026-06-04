use crate::components::player::Player;
use crate::managers::state_manager::State;
use crate::utilities::constants::{CAMERA_SPEED, CANVAS_HEIGHT, CANVAS_WIDTH, CLEAR_COLOR};
use bevy::camera::ScalingMode;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow.run_if(in_state(State::Playing)));
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_WIDTH as f32,
                min_height: CANVAS_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
    ));
}

fn camera_follow(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    camera.translation = camera
        .translation
        .lerp(player.translation, time.delta_secs() * CAMERA_SPEED);
}
