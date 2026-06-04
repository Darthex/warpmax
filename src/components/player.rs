use crate::managers::state_manager::State;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), spawn_player)
            .add_systems(
                Update,
                (player_movement, player_targeting)
                    .run_if(in_state(State::Playing)),
            );
    }
}

//struct //TAG
#[derive(Component)]
struct Player {
    speed: f32,
    rot: f32,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawns an entity and attaches following components on top of it
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(40.)),
            image: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {
            speed: 550.0,
            rot: f32::to_radians(450.0),
        },
    ));
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Single<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.into_inner();
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        let delta = direction * player.speed * time.delta_secs();

        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    } else {
    }
}

pub fn player_targeting(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    transform: Single<&mut Transform, With<Player>>,
) {
    let mut transform = transform;

    let (cam, cam_transform) = *camera_query;
    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = cam.viewport_to_world_2d(cam_transform, cursor_position)
    {
        transform.rotation = Quat::from_rotation_z(
            (cursor_world_pos - transform.translation.xy()).to_angle() - FRAC_PI_2,
        );
    };
}
