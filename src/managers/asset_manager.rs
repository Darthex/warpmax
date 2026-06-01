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
    pub cursor_red_sprite: Handle<Image>,
    pub cursor_purple_sprite: Handle<Image>,

    //audio
    pub main_menu_bg: Handle<AudioSource>,
    pub hover_sfx: Handle<AudioSource>,
    pub click_sfx: Handle<AudioSource>,
    pub whoosh_sfx: Handle<AudioSource>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        logo: asset_server.load("sprites/logo.png"),
        player_sprite: asset_server.load("sprites/player.png"),
        cursor_red_sprite: asset_server.load("sprites/cursor_red.png"),
        cursor_purple_sprite: asset_server.load("sprites/cursor_purple.png"),
        main_menu_bg: asset_server.load("audio/mm_bg.mp3"),
        hover_sfx: asset_server.load("audio/hover_sfx.mp3"),
        click_sfx: asset_server.load("audio/click_sfx.mp3"),
        whoosh_sfx: asset_server.load("audio/whoosh_sfx.mp3"),
    });
}
