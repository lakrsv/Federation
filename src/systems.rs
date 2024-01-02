use bevy::{ecs::{system::{Commands, Query, ResMut, Res}, query::With}, sprite::{MaterialMesh2dBundle, ColorMaterial, SpriteBundle}, render::{mesh::{shape, Mesh}, color::Color}, asset::{Assets, AssetServer}, transform::components::Transform, math::Vec3, prelude::default, core_pipeline::core_2d::Camera2dBundle, time::Time, log};
use bevy::prelude::*;

use crate::components::CelestialBody;

pub fn setup_viewport(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_planets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        CelestialBody{radius: 25.0, angular_velocity: -0.125}, 
        SpriteBundle{
        texture: asset_server.load("planet_1.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    },
));
}


pub fn move_planets(time: Res<Time>, mut planets: Query<(&CelestialBody, &mut Transform)>) {
    for ((celestial_body, mut transform)) in &mut planets {
        transform.rotate_z(celestial_body.angular_velocity * time.delta_seconds());
    }
}