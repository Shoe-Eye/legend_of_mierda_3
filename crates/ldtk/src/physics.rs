use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: bevy_rapier2d::dynamics::Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

pub fn setup_gravity(mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut rapier_config = rapier_config.single_mut().unwrap();
    rapier_config.gravity = Vec2::ZERO;
}
