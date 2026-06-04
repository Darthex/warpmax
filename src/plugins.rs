use crate::components::parallax::ParallaxPlugin;
use crate::components::player::PlayerPlugin;
use crate::components::player_attacks::PlayerAttacksPlugin;
use crate::core::camera::CameraPlugin;
use crate::core::window::{LWindowPlugin, get_window_plugin};
use crate::managers::asset_manager::AssetManagerPlugin;
use crate::managers::audio_manager::AudioManagerPlugin;
use crate::managers::state_manager::StateManagerPlugin;
use crate::scenes::game_scene::GameScenePlugin;
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
        .add_plugins(AudioManagerPlugin)
        .add_plugins(LWindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ParallaxPlugin)
        .add_plugins(MainMenuScenePlugin)
        .add_plugins(PlayerAttacksPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GameScenePlugin);
    }
}
