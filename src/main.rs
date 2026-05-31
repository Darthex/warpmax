mod core;
mod managers;
mod plugins;
mod scenes;
mod utilities;
mod components;

use crate::plugins::GamePlugins;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugins).run()
}
