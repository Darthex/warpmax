use crate::managers::state_manager::State;
use bevy::prelude::*;
use rand::Rng;

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::MainMenu), spawn_stars)
            .add_systems(Update, move_stars.run_if(in_state(State::MainMenu)));
    }
}

/*
DEVLOG: why normalized space? I was facing an issue with screen resize. so instead of storing the actual position of the star,
I store it in normalized space (0 to 1) and then convert it to actual position every frame.
This way, when the screen resizes, the stars will still be in the correct position.
*/

#[derive(Component)]
struct Star {
    layer: u8,
    norm_x: f32,
    norm_y: f32,
}

const LAYER_SPEEDS: [f32; 3] = [20., 45., 80.];
const LAYER_SIZES: [f32; 3] = [1., 2., 3.];
const LAYER_ALPHAS: [f32; 3] = [0.3, 0.6, 1.0];
const STARS_PER_LAYER: usize = 80;

fn spawn_stars(mut commands: Commands) {
    let mut rng = rand::rng();

    for layer in 0u8..3 {
        for _ in 0..STARS_PER_LAYER {
            commands.spawn((
                Sprite {
                    color: Color::srgba(1., 1., 1., LAYER_ALPHAS[layer as usize]),
                    custom_size: Some(Vec2::splat(LAYER_SIZES[layer as usize])),
                    ..default()
                },
                Transform::from_xyz(0., 0., -10.0 * layer as f32),
                Star {
                    layer,
                    norm_x: rng.random_range(0.0..1.0),
                    norm_y: rng.random_range(0.0..1.0),
                },
            ));
        }
    }
}

fn move_stars(
    mut stars: Query<(&mut Transform, &mut Star)>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    let (w, h) = (window.width(), window.height());
    for (mut transform, mut star) in &mut stars {
        let speed = LAYER_SPEEDS[star.layer as usize];
        star.norm_y -= (speed * time.delta_secs()) / h;

        // wrap around to the top
        if star.norm_y < 0.0 {
            star.norm_y = 1.0;
        }

        transform.translation.x = (star.norm_x - 0.5) * w;
        transform.translation.y = (star.norm_y - 0.5) * h;
    }
}
