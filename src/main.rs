mod core;
mod plugins;
mod scenes;
mod utilities;

use crate::plugins::GamePlugins;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(GamePlugins)
        .run()
}
