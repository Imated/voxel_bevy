mod chunk;
mod chunk_render_plugin;

use crate::chunk_render_plugin::ChunkRenderPlugin;
use bevy::DefaultPlugins;
use bevy::app::App;

fn main() {
    pretty_env_logger::init();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ChunkRenderPlugin::default())
        .run();
}
