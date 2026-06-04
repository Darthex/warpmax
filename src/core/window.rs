use crate::managers::asset_manager::Assets;
use crate::managers::state_manager::State;
use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, GAME_NAME};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{
    CursorGrabMode, CursorIcon, CursorOptions, CustomCursor, CustomCursorImage, PrimaryWindow,
    WindowMode, WindowResolution, WindowTheme,
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
        app.init_resource::<VirtualCursor>()
            .add_systems(PostStartup, init_cursor)
            .add_systems(OnEnter(State::Playing), setup_virtual_cursor)
            .add_systems(
                PreUpdate,
                update_virtual_cursor.run_if(in_state(State::Playing)),
            )
            .add_systems(Update, toggle_fullscreen)
            .add_systems(OnEnter(State::Loading), cursor_grab)
            .add_systems(OnExit(State::Playing), cursor_ungrab)
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

#[derive(Resource, Default)]
pub struct VirtualCursor {
    pub position: Vec2,
}

pub enum CursorType {
    Red,
    Purple,
}

#[derive(Event)]
pub struct CycleCursor {
    pub type_: CursorType,
}

fn init_cursor(
    mut commands: Commands,
    window: Single<Entity, With<PrimaryWindow>>,
    assets: Res<Assets>,
) {
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

fn setup_virtual_cursor(mut cursor: ResMut<VirtualCursor>) {
    cursor.position = Vec2::new(CANVAS_WIDTH as f32 / 2.0, CANVAS_HEIGHT as f32 / 2.0);
}

fn update_virtual_cursor(
    mut motion: MessageReader<MouseMotion>,
    mut cursor: ResMut<VirtualCursor>,
    window: Single<&Window>,
) {
    for event in motion.read() {
        cursor.position.x += event.delta.x;
        cursor.position.y -= event.delta.y;
    }
    cursor.position.x = cursor.position.x.clamp(0.0, window.width());
    cursor.position.y = cursor.position.y.clamp(0.0, window.height());
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

fn cursor_grab(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Confined;
}

fn cursor_ungrab(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}
