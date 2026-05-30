use crate::core::camera::CameraPlugin;
use crate::core::window::get_window_plugin;
use bevy::prelude::*;

// Plugin registry
pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(get_window_plugin()))
            .add_plugins(CameraPlugin);
    }
}
