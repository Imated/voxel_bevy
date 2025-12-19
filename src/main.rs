use bevy::app::App;
use bevy::DefaultPlugins;

fn main() {
    pretty_env_logger::init();
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
