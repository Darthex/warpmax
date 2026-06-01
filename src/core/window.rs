use crate::managers::asset_manager::Assets;
use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, GAME_NAME};
use bevy::prelude::*;
use bevy::window::{
    CursorIcon, CustomCursor, CustomCursorImage, WindowMode, WindowResolution, WindowTheme,
};

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
        app.add_systems(PostStartup, init_cursor)
            .add_systems(Update, toggle_fullscreen);
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

fn init_cursor(mut commands: Commands, window: Single<Entity, With<Window>>, assets: Res<Assets>) {
    let c_sprite = assets.cursor_sprite.clone();
    let c_icon = CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
        handle: c_sprite,
        hotspot: (0, 0),
        ..default()
    }));
    commands.entity(*window).insert(c_icon);
}
