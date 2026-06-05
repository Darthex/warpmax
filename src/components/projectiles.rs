use crate::managers::state_manager::State;
use crate::utilities::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;

pub struct ProjectilesPlugin;
impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_projectiles, despawn_outside_arena).run_if(in_state(State::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub direction: Vec2,
    pub damage: u32,
}

fn move_projectiles(mut query: Query<(&Projectile, &mut Transform)>, time: Res<Time>) {
    for (projectile, mut transform) in query.iter_mut() {
        let movement = projectile.direction * projectile.speed * time.delta_secs();
        transform.translation += movement.extend(0.);
    }
}

fn despawn_outside_arena(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (entity, transform) in &bullets {
        if transform.translation.x.abs() > ARENA_WIDTH / 2.0
            || transform.translation.y.abs() > ARENA_HEIGHT / 2.0
        {
            commands.entity(entity).despawn();
        }
    }
}
