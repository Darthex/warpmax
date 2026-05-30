use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, GAME_NAME};
use bevy::prelude::*;
use bevy::window::WindowResolution;

// Plugin registry
pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(GAME_NAME),
                resolution: WindowResolution::new(CANVAS_WIDTH, CANVAS_HEIGHT),
                ..default()
            }),
            ..default()
        }));
    }
}
