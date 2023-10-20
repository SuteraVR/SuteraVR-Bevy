use bevy::prelude::*;

mod player;
mod world;

use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin))
        .run();
}
