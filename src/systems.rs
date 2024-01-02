use bevy::{
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
    log::info,
    math::{Vec2, Vec3},
    prelude::default,
    render::camera::{OrthographicProjection, Camera},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
};
use bevy_rapier2d::{dynamics::{RigidBody, Velocity, ExternalForce, GravityScale, LockedAxes}, geometry::Collider};

use crate::components::{CelestialBody, PlayerCamera, PlayerVehicle};

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera()));
    commands.spawn((
        PlayerVehicle { speed: 16., rotation_speed: 4. },
        SpriteBundle {
            texture: asset_server.load("ship.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(
                1. / 8.,
                1. / 8.,
                1. / 8.,
            )),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_Z,
        GravityScale(0.),
        ExternalForce {
            force: Vec2::new(0., 0.),
            torque: 0.,
        },
        Collider::ball(512./4.),
    ));
}

pub fn setup_planets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        CelestialBody {
            radius: 25.0,
            angular_velocity: -0.125,
        },
        SpriteBundle {
            texture: asset_server.load("planet_1.png"),
            transform: Transform::from_xyz(512., 0., 0.).with_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::ball(256.),
    ));
}

pub fn rotate_planets(time: Res<Time>, mut planets: Query<(&CelestialBody, &mut Transform)>) {
    for (celestial_body, mut transform) in &mut planets {
        transform.rotate_z(celestial_body.angular_velocity * time.delta_seconds());
    }
}

pub fn zoom_camera(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query_camera: Query<&mut OrthographicProjection, With<PlayerCamera>>,
) {
    let mut scale = 0.;
    if keys.pressed(KeyCode::Z) {
        // zoom in
        scale = -0.5 * time.delta_seconds();
    } else if keys.pressed(KeyCode::X) {
        // zoom in
        scale = 0.5 * time.delta_seconds();
    }

    if scale != 0. {
        let mut projection = query_camera.single_mut();
        if projection.scale + scale > 1.75 {
            projection.scale = 1.75;
        } else if projection.scale + scale < 0.25 {
            projection.scale = 0.25;
        } else {
            projection.scale += scale;
        }
    }
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query_player: Query<(&PlayerVehicle, &mut Transform, &mut ExternalForce)>,
) {
    let (player, mut transform, mut external_force) = query_player.single_mut();

    let mut movement = Vec2::new(0., 0.);
    if keys.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if keys.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if keys.pressed(KeyCode::D) {
        movement.x += 1.;
    }

    //let mut moveY = Vec2::new(0., movement.y * player.speed);

    let movement_direction = transform.rotation * Vec3::Y;
    external_force.force = movement.y * player.speed * Vec2::new(movement_direction.x, movement_direction.y);
    transform.rotate_z(-movement.x * player.rotation_speed * time.delta_seconds());
}

pub fn camera_follow(player: Query<(&PlayerVehicle, &Transform)>, mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerVehicle>)>) {
    let player = player.single();
    let mut camera_transform = camera.single_mut();

    camera_transform.translation.x = player.1.translation.x;
    camera_transform.translation.y = player.1.translation.y;
}
