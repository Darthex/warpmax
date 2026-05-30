mod plugins;
mod utilities;
mod scenes;

use crate::plugins::GamePlugins;
use crate::utilities::constants::CLEAR_COLOR;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(GamePlugins)
        .run()
}
