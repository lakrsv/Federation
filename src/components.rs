use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CelestialBody {
    pub radius: f32,
    pub angular_velocity: f32,
}

#[derive(Component)]
pub struct PlayerCamera();

#[derive(Component)]
pub struct PlayerVehicle {
    pub speed: f32,
    pub rotation_speed: f32,
}
