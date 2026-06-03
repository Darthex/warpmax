use bevy::prelude::*;
use crate::managers::state_manager::State;

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
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,)
{   //spawns an entity and attaches following components on top of it
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(50.)),
            image: asset_server.load("sprites/player.png"),  
            ..default()
        },  
        Transform::from_translation(Vec3::ZERO),
        Player {
            speed: 600.0,
        },
    ));
}


//iterates over the Transform and AnimationState components added to Entity
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>, 
    query: Single<(&Player, &mut Transform)>,)
{
    let (player, mut transform) = query.into_inner();
    let mut direction = Vec2::ZERO;

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

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * player.speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
    else {
    }
}





