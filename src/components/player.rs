use crate::managers::state_manager::State;
use crate::scenes::game_scene::ClampRadius;
use crate::utilities::animations::ease_out_back;
use crate::utilities::constants::{ARENA_BORDER_WIDTH, ARENA_HEIGHT, LOADING_TIMER};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Loading), spawn_player)
            .add_systems(
                Update,
                animate_player_entry.run_if(in_state(State::Loading)),
            )
            .add_systems(Update, player_movement.run_if(in_state(State::Playing)));
    }
}

//structs & enums
#[derive(Component)]
pub struct Player {
    speed: f32,
    rot: f32,
}

#[derive(Component)]
struct PlayerEntryAnimation {
    timer: Timer,
    start_y: Option<f32>,
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
        ClampRadius(40.),
        Transform::from_xyz(0., -ARENA_HEIGHT / 2. - ARENA_BORDER_WIDTH - 40., 0.),
        PlayerEntryAnimation {
            timer: Timer::from_seconds(LOADING_TIMER as f32, TimerMode::Once),
            start_y: None,
        },
    ));
}

//iterates over the Transform and AnimationState components added to Entity
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Single<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.into_inner();
    let mut direction = Vec3::ZERO;
    let mut rotation = 0.0;

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
    if input.pressed(KeyCode::ArrowLeft) {
        rotation += 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        rotation -= 1.0;
    }
    //sets axis for rotation (current axisZ)
    transform.rotate_z(rotation * player.rot * time.delta_secs());

    if direction != Vec3::ZERO {
        let delta = direction * player.speed * time.delta_secs();
        let facing_delta = transform.rotation * delta.normalize();

        //gives player movement
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        //gives player rotation at (direction speed = from input)
        transform.translation += facing_delta;
    } else {
    }
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
