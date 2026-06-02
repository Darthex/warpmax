use crate::managers::state_manager::State;
use crate::utilities::constants::{ARENA_BORDER_WIDTH, ARENA_COLOR, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;

pub struct GameScenePlugin;
impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), spawn_arena);
    }
}

fn spawn_arena(mut commands: Commands) {
    // Left, bottom, right, top
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_BORDER_WIDTH,
                ARENA_HEIGHT + (ARENA_BORDER_WIDTH * 2.),
            )),
            ..default()
        },
        Transform::from_xyz((-ARENA_WIDTH / 2.) - ARENA_BORDER_WIDTH, 0., 0.),
    ));
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_WIDTH + (ARENA_BORDER_WIDTH * 2.0),
                ARENA_BORDER_WIDTH,
            )),
            ..default()
        },
        Transform::from_xyz(0., (-ARENA_HEIGHT / 2.) - ARENA_BORDER_WIDTH, 0.),
    ));
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_BORDER_WIDTH,
                ARENA_HEIGHT + (ARENA_BORDER_WIDTH * 2.),
            )),
            ..default()
        },
        Transform::from_xyz((ARENA_WIDTH / 2.) + ARENA_BORDER_WIDTH, 0., 0.),
    ));
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_WIDTH + (ARENA_BORDER_WIDTH * 2.),
                ARENA_BORDER_WIDTH,
            )),
            ..default()
        },
        Transform::from_xyz(0., (ARENA_HEIGHT / 2.) + ARENA_BORDER_WIDTH, 0.),
    ));
}
