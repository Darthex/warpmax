use crate::core::window::VirtualCursor;
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
    pub aim_dir: Vec2,
    pub shoot: bool,
    pub shoot_just_pressed: bool,
    pub pause_just_pressed: bool,
}

fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepads: Query<&Gamepad>,
    cursor: Res<VirtualCursor>,
    mut manager: ResMut<InputManager>,
    window: Single<&Window>,
) {
    manager.shoot_just_pressed = false;
    manager.pause_just_pressed = false;
    manager.move_dir = Vec2::ZERO;
    manager.aim_dir = Vec2::ZERO;

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

    let screen_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let aim = Vec2::new(
        cursor.position.x - screen_center.x,
        cursor.position.y - screen_center.y,
    );
    if aim.length() > 1.0 {
        manager.aim_dir = aim.normalize();
    }

    // if let Some(gamepad) = gamepads.iter().next() {
    //     let lx = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
    //     let ly = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
    //     let left = Vec2::new(lx, ly);
    //     if left.length() > 0.1 {
    //         manager.move_dir += left;
    //     }
    //
    //     let rx = gamepad.get(GamepadAxis::RightStickX).unwrap_or(0.0);
    //     let ry = gamepad.get(GamepadAxis::RightStickY).unwrap_or(0.0);
    //     let right = Vec2::new(rx, ry);
    //     if right.length() > 0.5 {
    //         manager.aim_dir = right.normalize();
    //         manager.shoot = true
    //     } else {
    //         manager.shoot = false
    //     }
    //
    //     if gamepad.just_pressed(GamepadButton::Start) {
    //         manager.pause_just_pressed = true;
    //     }
    // }

    if manager.move_dir != Vec2::ZERO {
        manager.move_dir = manager.move_dir.clamp_length_max(1.0);
    }
}
