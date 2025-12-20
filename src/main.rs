mod chunk;
mod chunk_render_plugin;

use bevy::DefaultPlugins;
use bevy::app::App;
use crate::chunk_render_plugin::ChunkRenderPlugin;

fn main() {
    pretty_env_logger::init();
    App::new().add_plugins(DefaultPlugins).add_plugins(ChunkRenderPlugin::default()).run();
}
