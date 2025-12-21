mod chunk;
mod chunk_render_plugin;

use crate::chunk_render_plugin::ChunkRenderPlugin;
use bevy::app::App;
use bevy::DefaultPlugins;
use bevy_flycam::PlayerPlugin;

fn main() {
    pretty_env_logger::init();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ChunkRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .run();
}
