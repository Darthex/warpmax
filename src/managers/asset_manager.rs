use bevy::prelude::*;

pub struct AssetManagerPlugin;
impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
pub struct Assets {
    // sprites
    pub logo: Handle<Image>,
    pub player_sprite: Handle<Image>,

    //audio
    pub main_menu_bg: Handle<AudioSource>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        logo: asset_server.load("sprites/logo.png"),
        player_sprite: asset_server.load("sprites/player.png"),
        main_menu_bg: asset_server.load("audio/mm_bg.mp3"),
    });
}
