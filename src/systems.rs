use bevy::{
    asset::{AssetServer},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::{With, Without, Added},
        system::{Commands, Query, Res}, entity::Entity, world::World,
    },
    input::{keyboard::KeyCode, Input},
    math::{Vec2, Vec3, vec3},
    prelude::default,
    render::{camera::{Camera, OrthographicProjection}, color::Color},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform, gizmos::gizmos::Gizmos,
};
use bevy_rapier2d::{
    dynamics::{ExternalForce, GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::Collider,
};

use crate::components::{CelestialBody, PlayerCamera, PlayerVehicle, OrbitParent, TeamRedHomePlanet, OrbitChild};

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, home_planet_query: Query<Entity, With<TeamRedHomePlanet>>) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera()));
    commands.spawn((
        OrbitChild{},
        PlayerVehicle {
            speed: 8.,
            boost_modifier: 2.0,
            rotation_speed: 4.,
        },
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
        Velocity {linvel: Vec2::ZERO, angvel: 0.0},
        ExternalForce {
            force: Vec2::new(0., 0.),
            torque: 0.,
        },
        Collider::ball(512. / 4.),
    ));

}

pub fn setup_home_planets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TeamRedHomePlanet,
        CelestialBody {
            radius: 25.0,
            angular_velocity: -0.125,
            mass: 10.0,
        },
        OrbitParent {},
        SpriteBundle {
            texture: asset_server.load("planet_1.png"),
            transform: Transform::from_xyz(512., 0., 0.).with_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::ball(256.),
    ));

    commands.spawn((
        TeamRedHomePlanet,
        CelestialBody {
            radius: 25.0,
            angular_velocity: -0.125,
            mass: 10.0,
        },
        OrbitParent {},
        SpriteBundle {
            texture: asset_server.load("planet_1.png"),
            transform: Transform::from_xyz(512., 2048., 0.).with_scale(Vec3::new(1.0, 1.0, 1.0)),
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
        scale = -1.0 * time.delta_seconds();
    } else if keys.pressed(KeyCode::X) {
        // zoom in
        scale = 1.0 * time.delta_seconds();
    }

    if scale != 0. {
        let mut projection = query_camera.single_mut();
        if projection.scale + scale > 2.5 {
            projection.scale = 2.5;
        } else if projection.scale + scale < 0.25 {
            projection.scale = 0.25;
        } else {
            projection.scale += scale * projection.scale;
        }
    }
}

pub fn move_player(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&PlayerVehicle, &mut Transform, &mut ExternalForce)>,
) {
    let (player, mut transform, mut external_force) = player.single_mut();

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

    let mut modifier = 1.0;
    if keys.pressed(KeyCode::ShiftLeft) {
        modifier = player.boost_modifier;
    }

    let movement_direction = transform.rotation * Vec3::Y;
    external_force.force =
        movement.y * player.speed * modifier * Vec2::new(movement_direction.x, movement_direction.y).normalize();
    transform.rotate_z(-movement.x * player.rotation_speed * time.delta_seconds());
}

pub fn camera_follow(
    player: Query<(&PlayerVehicle, &Transform)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerVehicle>)>,
) {
    let player = player.single();
    let mut camera_transform = camera.single_mut();

    camera_transform.translation.x = player.1.translation.x;
    camera_transform.translation.y = player.1.translation.y;
}

pub fn orbit_objects(mut orbit_children: Query<(&mut Velocity, &Transform, With<OrbitChild>)>, orbit_parents: Query<(&Transform, With<OrbitParent>, Without<OrbitChild>)>, time: Res<Time>) {
    for mut orbit_child in orbit_children.iter_mut() {
        for orbit_parent in orbit_parents.iter() {
        let child_transform = orbit_child.1;
        
        let gravity = calculate_gravity(child_transform.translation, orbit_parent.0.translation, 1.0, 1_000_000.0);
        orbit_child.0.linvel += gravity * time.delta_seconds();
        }
    }
}

pub fn draw_player_orbit(player_query: Query<(&PlayerVehicle, &Transform, &Velocity, With<OrbitChild>)>, orbit_parents: Query<(&Transform, With<OrbitParent>, Without<OrbitChild>)>, mut gizmos: Gizmos, time: Res<Time>) {
    let player = player_query.single();
    let mut current_position = player.1.translation;
    let mut current_velocity = player.2.linvel;
    'outer: for _ in 0..10_000 { 
        for orbit_parent in orbit_parents.iter() {
            if current_position.distance(orbit_parent.0.translation) < 256.0 {
                break 'outer;
            }
        }
        for orbit_parent in orbit_parents.iter() {
        let gravity = calculate_gravity(current_position, orbit_parent.0.translation, 1.0, 1_000_000.0);
        current_velocity += gravity * time.delta_seconds();
        }
        let next_position = current_position + Vec3::new(current_velocity.x, current_velocity.y, 0.0) * time.delta_seconds();
        gizmos.line_2d(Vec2::new(current_position.x, current_position.y), Vec2::new(next_position.x, next_position.y), Color::WHITE);
        current_position = next_position;
    }
}

fn calculate_gravity(object_position: Vec3, target_position: Vec3, object_mass: f32, target_mass: f32) -> Vec2{
    let G = 10.0;
    let distance = object_position.distance_squared(target_position);
    if distance > 1_000_000.0 {
        return Vec2::ZERO;
    }
    let move_direction = (target_position - object_position).normalize();
    let force = (G * object_mass * target_mass) / distance;
    return Vec2::new(move_direction.x, move_direction.y) * force;
}
