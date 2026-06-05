use crate::managers::state_manager::State;
use bevy::prelude::*;

pub struct InputManagerPlugin;
impl Plugin for InputManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputManager>()
            .add_systems(PreUpdate, read_input.run_if(in_state(State::Playing)));
    }
}

#[derive(Resource, Default)]
pub struct InputManager {
    pub move_dir: Vec2,
    pub shoot: bool,
    pub shoot_just_pressed: bool,
    pub pause_just_pressed: bool,
}

fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepads: Query<&Gamepad>,
    mut manager: ResMut<InputManager>,
) {
    manager.shoot_just_pressed = false;
    manager.pause_just_pressed = false;
    manager.move_dir = Vec2::ZERO;

    if keys.just_pressed(KeyCode::Escape) {
        manager.pause_just_pressed = true;
    }

    manager.shoot = mouse.pressed(MouseButton::Left);
    if mouse.just_pressed(MouseButton::Left) {
        manager.shoot_just_pressed = true;
    }

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    if dir != Vec2::ZERO {
        manager.move_dir += dir.normalize();
    }

    if let Some(gamepad) = gamepads.iter().next() {
        let lx = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let ly = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
        let left = Vec2::new(lx, ly);
        if left.length() > 0.1 {
            manager.move_dir += left;
        }

        let rt = gamepad.get(GamepadAxis::RightZ).unwrap_or(0.0);
        if rt > 0.5 {
            manager.shoot = true;
        }
        if gamepad.just_pressed(GamepadButton::RightTrigger2) {
            manager.shoot_just_pressed = true;
        }
        if gamepad.just_pressed(GamepadButton::Start) {
            manager.pause_just_pressed = true;
        }
    }

    if manager.move_dir != Vec2::ZERO {
        manager.move_dir = manager.move_dir.clamp_length_max(1.0);
    }
}
