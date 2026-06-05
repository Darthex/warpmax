use crate::managers::asset_manager::Assets;
use crate::managers::input_manager::InputManager;
use crate::managers::state_manager::State;
use crate::scenes::game_scene::ClampRadius;
use crate::utilities::animations::ease_out_back;
use crate::utilities::constants::{
    ARENA_BORDER_WIDTH, ARENA_HEIGHT, DAMPING, LOADING_TIMER, PLAYER_MOVEMENT_SPEED,
    PLAYER_ROTATION_SPEED,
};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Loading), spawn_player)
            .add_systems(
                Update,
                animate_player_entry.run_if(in_state(State::Loading)),
            )
            .add_systems(Update, (move_player).run_if(in_state(State::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
struct PlayerEntryAnimation {
    timer: Timer,
    start_y: Option<f32>,
}

fn spawn_player(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(40.)),
            image: assets.player_sprite.clone(),
            ..default()
        },
        Player,
        Velocity::default(),
        ClampRadius(40.),
        Transform::from_xyz(0., -ARENA_HEIGHT / 2. - ARENA_BORDER_WIDTH - 40., 0.),
        PlayerEntryAnimation {
            timer: Timer::from_seconds(LOADING_TIMER as f32, TimerMode::Once),
            start_y: None,
        },
    ));
}

fn move_player(
    input: Res<InputManager>,
    time: Res<Time>,
    mut smooth_dir: Local<Vec2>,
    mut player: Single<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (ref mut transform, ref mut vel) = *player;
    let dt = time.delta_secs();

    *smooth_dir = smooth_dir.lerp(input.move_dir, (10.0 * dt).min(1.0));

    if smooth_dir.length() > 0.01 {
        vel.0 += *smooth_dir * PLAYER_MOVEMENT_SPEED;
        let target = Quat::from_rotation_z(smooth_dir.to_angle() - FRAC_PI_2);
        transform.rotation = transform.rotation.slerp(target, dt * PLAYER_ROTATION_SPEED);
    }

    vel.0 *= 1.0 - (DAMPING * dt).min(1.0);
    vel.0 = vel.0.clamp_length_max(PLAYER_MOVEMENT_SPEED);
    transform.translation.x += vel.0.x * dt;
    transform.translation.y += vel.0.y * dt;
}

fn animate_player_entry(
    mut commands: Commands,
    mut player: Single<(Entity, &mut Transform, &mut PlayerEntryAnimation)>,
    time: Res<Time>,
) {
    let (entity, ref mut transform, ref mut entry) = *player;

    if entry.start_y.is_none() {
        entry.start_y = Some(transform.translation.y);
    }
    let start_y = entry.start_y.unwrap();
    // let end_y = -(ARENA_HEIGHT / 2.0) * 0.5;
    let end_y = 0.;

    entry.timer.tick(time.delta());
    let progress = entry.timer.fraction();
    let eased = ease_out_back(progress, Some(1.2)); // Little bounce on landing

    transform.translation.y = start_y + (end_y - start_y) * eased;

    if entry.timer.just_finished() {
        commands.entity(entity).remove::<PlayerEntryAnimation>();
    }
}
