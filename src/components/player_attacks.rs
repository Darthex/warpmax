/*use bevy::prelude::*;
use bevy::color::palettes::basic::RED;
use std::f32::consts::FRAC_PI_2;
use crate::managers::state_manager::State;
use crate::components::player::{player_movement, player_targeting};

pub struct PlayerAttacksPlugin;

impl Plugin for PlayerAttacksPlugin {
    fn build(&self,app: &mut App) {
        app.add_systems(OnEnter(State::Playing), attack_button)
            .add_systems(Update, attack_button.run_if(in_state(State::Playing)));
    }
}

#[derive(Component)]
pub struct Attacks {
    speed: f32,
    direction: Vec2,
}

pub fn attack_button(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        commands.spawn((
            Mesh2d(meshes.add(Capsule2d::new(4.0, 4.0))),
            MeshMaterial2d(materials.add(Color::from(RED))),
            Attacks {speed: 1000.0},
        ));
    } else {
    }
}

fn aiming(
    time: Res<Time>,
    mut transform: Single<(&mut Transform, &Attacks)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (mut transform, attack) = transform.into_inner();
    let mut input = Vec3::ZERO;

    if mouse.pressed(MouseButton::Left) {
        input.x += 1.0;
    }

    let projectile = input * attack.speed * time.delta_secs();
    transform.translation.x += projectile.x;
    transform.translation.y += projectile.y;
}

*/
