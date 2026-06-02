use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, CLEAR_COLOR};
use bevy::camera::ScalingMode;
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
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
