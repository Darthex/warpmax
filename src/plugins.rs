use crate::core::camera::CameraPlugin;
use crate::core::window::get_window_plugin;
use crate::managers::asset_manager::AssetManagerPlugin;
use crate::managers::state_manager::StateManagerPlugin;
use crate::scenes::main_menu_scene::MainMenuScenePlugin;
use bevy::prelude::*;

// Plugin registry
pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(get_window_plugin())
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(StateManagerPlugin)
        .add_plugins(AssetManagerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MainMenuScenePlugin);
    }
}
