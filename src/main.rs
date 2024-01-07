use bevy::prelude::*;

use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use systems::*;
mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(128.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_home_planets))
        .add_systems(PostStartup, (setup_player))
        .add_systems(
            Update,
            (rotate_planets, zoom_camera, move_player, camera_follow, orbit_objects, draw_player_orbit),
        )
        .run();
}
