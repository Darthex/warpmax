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
    pub player_sprite: Handle<Image>,
    pub menu_background: Handle<Image>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        player_sprite: asset_server.load("sprites/player.png"),
        menu_background: asset_server.load("sprites/mm_bg.jpg"),
    });
}
