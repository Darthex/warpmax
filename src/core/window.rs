use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, GAME_NAME};
use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution, WindowTheme};

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

pub struct LWindowPlugin;
impl Plugin for LWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_fullscreen);
    }
}

fn toggle_fullscreen(mut window: Single<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F11) {
        if window.mode == WindowMode::Windowed {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
        } else {
            window.mode = WindowMode::Windowed;
        }
    }
}
