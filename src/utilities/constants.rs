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
pub const ARENA_COLOR: Color = Color::linear_rgb(3.9, 0.52, 0.39);
