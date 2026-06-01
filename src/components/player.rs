use bevy::prelude::*;
use crate::managers::state_manager::State;
use crate::managers::asset_manager::Assets;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(State::Playing)));
    }
}


//constant values
const SPEED: f32 = 100.0;


//structs & enums
#[derive(Component)]
struct Player;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    is_moving: bool,
    is_idle: bool,
} 


fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,)
{
    let facing = Facing::Up;
    let is_moving = false;
    let is_idle = true;

    //spawns an entity and attaches following components on top of it
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/player.png")),
        AnimationState { facing, is_moving, is_idle },
        Transform::from_translation(Vec3::ZERO),
        Player,
    ));
}


//iterates over the Transform and AnimationState components added to Entity
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>, 
    mut transform: Single<&mut Transform, With<Player>>,)
{
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
        
        let delta = direction.normalize() * SPEED * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
    else {
    }  
}





