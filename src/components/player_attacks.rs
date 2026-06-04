use bevy::prelude::*;
use bevy::{color::palettes::basic::RED, prelude::*};
use crate::managers::state_manager::State;
use crate::components::player::{player_movement, player_targeting};

pub struct PlayerAttacksPlugin;

impl Plugin for PlayerAttacksPlugin {
    fn build(&self,app: &mut App) {
        app.add_systems(OnEnter(State::Playing), spawn_attacks)
            .add_systems(Update, firing_attacks.run_if(in_state(State::Playing)));
    }
}

#[derive(Component)]
struct Attacks {
    speed: f32,
}

fn spawn_attacks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,  mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(4.0, 4.0))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_translation(player_movement[2]),
        Attacks {
            speed: 1000.0,
        },
    ));
}

fn firing_attacks(
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    attack: Single<(&mut Transform, &Attacks)>,
) {
    let (mut transform, attack) = attack.into_inner();
    let mut input = Vec3::ZERO;

    if mouse.pressed(MouseButton::Left) {
        input += 1.0;
    }
    if mouse.pressed(MouseButton::Right) {
        input += 1.0;
    } 
    
    let attack_velocity = input * attack.speed * time.delta_secs();
    transform.translation += attack_velocity 
}


