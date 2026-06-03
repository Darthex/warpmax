use crate::managers::state_manager::State;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(State::Playing)));
    }
}
//constant values
//structs & enums
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
