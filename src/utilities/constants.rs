use bevy::color::Color;
use bevy::text::TextColor;

pub const GAME_NAME: &str = "Warpmax";
pub const CLEAR_COLOR: Color = Color::srgb_u8(10, 10, 10);
pub const CANVAS_WIDTH: u32 = 1920;
pub const CANVAS_HEIGHT: u32 = 1080;

pub const TITLE_HEIGHT: f32 = 256.;
pub const TITLE_WIDTH: f32 = 600.;
pub const FONT_SIZE: f32 = 32.;
pub const BUTTON_COLOR: TextColor = TextColor(Color::srgb_u8(255, 255, 255));
pub const BUTTON_ACTION_COLOR: TextColor = TextColor(Color::srgb_u8(211, 211, 211));

pub const SOUNDTRACK_FADE_TIME: f32 = 3.;

pub const ARENA_WIDTH: f32 = 1900.;
pub const ARENA_HEIGHT: f32 = 1060.;
pub const ARENA_BORDER_WIDTH: f32 = 2.;

pub const LOADING_TIMER: u32 = 3;
pub const CAMERA_SPEED: f32 = 8.;

pub const PLAYER_MOVEMENT_SPEED: f32 = 1000.;
pub const PLAYER_ROTATION_SPEED: f32 = 100.;
pub const DAMPING: f32 = 5.0;
pub const PLAYER_ATTACK_RATE: f32 = 0.15;
pub const PLAYER_SIZE: f32 = 40.;

pub const BLOOM_RED: Color = Color::linear_rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
pub const BLOOM_YELLOW: Color = Color::linear_rgb(COLOR_YELLOW.0, COLOR_YELLOW.1, COLOR_YELLOW.2);

pub const COLOR_RED: (f32, f32, f32) = (3.9, 0.52, 0.39);
pub const COLOR_YELLOW: (f32, f32, f32) = (8., 6., 0.);
