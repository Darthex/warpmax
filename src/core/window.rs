use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, GAME_NAME};
use bevy::prelude::{Window, WindowPlugin, default};
use bevy::window::{WindowResolution, WindowTheme};

pub fn get_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: String::from(GAME_NAME),
            resolution: WindowResolution::new(CANVAS_WIDTH, CANVAS_HEIGHT),
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}
