use crate::managers::asset_manager::Assets;
use bevy::prelude::*;

pub struct AudioManagerPlugin;
impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
    }
}

fn setup(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        AudioPlayer::new(assets.main_menu_bg.clone()),
        PlaybackSettings::LOOP,
    ));
}
