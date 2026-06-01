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
            .add_systems(Update, toggle_fullscreen)
            .add_observer(cycle_cursor);
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

#[derive(Resource)]
struct CursorIcons(Vec<CursorIcon>);

pub enum CursorType {
    Red,
    Purple,
}

#[derive(Event)]
pub struct CycleCursor {
    pub type_: CursorType,
}

fn init_cursor(mut commands: Commands, window: Single<Entity, With<Window>>, assets: Res<Assets>) {
    let cursor_icons = CursorIcons(vec![
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: assets.cursor_red_sprite.clone(),
            hotspot: (0, 0),
            ..default()
        })),
        CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: assets.cursor_purple_sprite.clone(),
            hotspot: (0, 0),
            ..default()
        })),
    ]);
    commands.entity(*window).insert(cursor_icons.0[0].clone());
    commands.insert_resource(cursor_icons);
}

fn cycle_cursor(
    event: On<CycleCursor>,
    mut cursor: Single<&mut CursorIcon>,
    cursor_icons: Res<CursorIcons>,
) {
    let next_icon = match event.type_ {
        CursorType::Red => &cursor_icons.0[0],
        CursorType::Purple => &cursor_icons.0[1],
    };
    **cursor = next_icon.clone();
}
