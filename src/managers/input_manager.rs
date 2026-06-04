use crate::core::window::VirtualCursor;
use crate::managers::state_manager::State;
use bevy::prelude::*;

pub struct InputManagerPlugin;
impl Plugin for InputManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputManager>()
            .init_resource::<ControlScheme>()
            .add_systems(PreUpdate, read_input.run_if(in_state(State::Playing)))
            .add_systems(
                Update,
                cycle_control_scheme.run_if(in_state(State::Playing)),
            );
    }
}

#[derive(Resource, Default, PartialEq, Clone)]
pub enum ControlScheme {
    #[default]
    Spacecraft,
    TwinStick,
}

#[derive(Resource, Default)]
pub struct InputManager {
    pub move_dir: Vec2,
    pub aim_dir: Vec2,
    pub rotate: f32,
    pub thrust: f32,
    pub shoot: bool,
    pub shoot_just_pressed: bool,
    pub pause_just_pressed: bool,
}

fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    scheme: Res<ControlScheme>,
    cursor: Res<VirtualCursor>,
    mut manager: ResMut<InputManager>,
    window: Single<&Window>,
) {
    manager.shoot_just_pressed = false;
    manager.pause_just_pressed = false;

    if keys.just_pressed(KeyCode::Escape) {
        manager.pause_just_pressed = true;
    }
    manager.shoot = mouse.pressed(MouseButton::Left);
    if mouse.just_pressed(MouseButton::Left) {
        manager.shoot_just_pressed = true;
    }

    match *scheme {
        ControlScheme::Spacecraft => {
            manager.move_dir = Vec2::ZERO;
            manager.aim_dir = Vec2::ZERO;

            let mut rotate = 0.0;
            if keys.pressed(KeyCode::KeyA) {
                rotate += 1.0;
            }
            if keys.pressed(KeyCode::KeyD) {
                rotate -= 1.0;
            }
            manager.rotate = rotate;

            let mut thrust = 0.0;
            if keys.pressed(KeyCode::KeyW) {
                thrust += 1.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                thrust -= 1.0;
            }
            manager.thrust = thrust;
        }

        ControlScheme::TwinStick => {
            manager.rotate = 0.0;
            manager.thrust = 0.0;

            let mut move_dir = Vec2::ZERO;
            if keys.pressed(KeyCode::KeyW) {
                move_dir.y += 1.0;
            }
            if keys.pressed(KeyCode::KeyS) {
                move_dir.y -= 1.0;
            }
            if keys.pressed(KeyCode::KeyA) {
                move_dir.x -= 1.0;
            }
            if keys.pressed(KeyCode::KeyD) {
                move_dir.x += 1.0;
            }
            manager.move_dir = if move_dir != Vec2::ZERO {
                move_dir.normalize()
            } else {
                Vec2::ZERO
            };

            let screen_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            let aim = Vec2::new(
                cursor.position.x - screen_center.x,
                cursor.position.y - screen_center.y,
            );
            manager.aim_dir = if aim.length() > 1.0 {
                aim.normalize()
            } else {
                Vec2::ZERO
            };
        }
    }
}

// TODO: remove after settings screen
fn cycle_control_scheme(keys: Res<ButtonInput<KeyCode>>, mut scheme: ResMut<ControlScheme>) {
    if keys.just_pressed(KeyCode::Digit1) {
        *scheme = ControlScheme::Spacecraft;
        info!("Control scheme: Spacecraft");
    }
    if keys.just_pressed(KeyCode::Digit2) {
        *scheme = ControlScheme::TwinStick;
        info!("Control scheme: TwinStick");
    }
}
