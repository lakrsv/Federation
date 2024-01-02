use bevy::prelude::*;
use bevy::winit::WinitSettings;
use systems::*;
mod components;
mod systems;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        ImagePlugin::default_nearest(),
    ))
    // .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (setup_viewport, setup_planets))
        .add_systems(Update, move_planets)
        // .add_systems(Update, greet_people)
    .run();
}