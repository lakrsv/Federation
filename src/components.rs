use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CelestialBody {
    pub radius: f32,
    pub angular_velocity: f32,
}
