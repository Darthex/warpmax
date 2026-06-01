use crate::managers::asset_manager::Assets;
use crate::scenes::main_menu_scene::{ButtonClick, ButtonHover};
use bevy::prelude::*;

pub struct AudioManagerPlugin;
impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_observer(play_hover)
            .add_observer(play_click);
    }
}

fn setup(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        AudioPlayer::new(assets.main_menu_bg.clone()),
        PlaybackSettings::LOOP,
    ));
}

fn play_hover(_: On<ButtonHover>, mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        AudioPlayer::new(assets.hover_sfx.clone()),
        PlaybackSettings::DESPAWN,
    ));
}

fn play_click(_: On<ButtonClick>, mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        AudioPlayer::new(assets.click_sfx.clone()),
        PlaybackSettings::DESPAWN,
    ));
}
