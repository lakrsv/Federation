use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CelestialBody {
    pub radius: f32,
    pub angular_velocity: f32,
    pub mass: f32,
}

#[derive(Component)]
pub struct OrbitParent {}

#[derive(Component)]
pub struct OrbitChild {}

#[derive(Component)]
pub struct PlayerCamera();

#[derive(Component)]
pub struct PlayerVehicle {
    pub speed: f32,
    pub boost_modifier: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct TeamRedHomePlanet;

#[derive(Component)]
pub struct TeamBlueHomePlanet;
