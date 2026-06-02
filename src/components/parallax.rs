use crate::managers::state_manager::State;
use crate::utilities::constants::{CANVAS_HEIGHT, CANVAS_WIDTH};
use bevy::prelude::*;
use rand::{Rng, RngExt};

pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::MainMenu), spawn_stars)
            .add_systems(Update, move_stars);
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

const LAYER_SPEEDS: [f32; 3] = [20., 45., 60.];
const LAYER_SIZES: [f32; 3] = [1., 2., 3.];
const LAYER_ALPHAS: [f32; 3] = [0.15, 0.35, 0.65];
const STARS_PER_LAYER: usize = 80;

fn spawn_stars(mut commands: Commands) {
    let mut rng = rand::rng();

    for layer in 0u8..3 {
        for _ in 0..STARS_PER_LAYER {
            let size = rng
                .random_range(LAYER_SIZES[layer as usize] * 0.5..LAYER_SIZES[layer as usize] * 1.5);
            commands.spawn((
                Sprite {
                    color: random_star_color(&mut rng, layer),
                    custom_size: Some(Vec2::splat(size)),
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
    // Window dimensions become 0 when minimized — skip to avoid NaN propagation.
    if w == 0.0 || h == 0.0 {
        return;
    }

    // Compute the visible world dimensions, matching ScalingMode::AutoMin.
    // AutoMin picks the scale that satisfies both minimum dimensions, so the
    // world size is always >= CANVAS size regardless of window pixel size.
    let scale = (w / CANVAS_WIDTH as f32).min(h / CANVAS_HEIGHT as f32);
    let world_w = w / scale;
    let world_h = h / scale;

    for (mut transform, mut star) in &mut stars {
        let speed = LAYER_SPEEDS[star.layer as usize];

        // Recover any stars corrupted by a previous minimize (NaN/infinite position).
        if !star.norm_x.is_finite() {
            star.norm_x = 0.5;
        }
        if !star.norm_y.is_finite() {
            star.norm_y = 0.5;
        }

        star.norm_y -= (speed * time.delta_secs()) / world_h;
        star.norm_x += (speed * time.delta_secs()) / world_w;

        // wrap around
        if star.norm_y < 0.0 {
            star.norm_y = 1.0;
        }
        if star.norm_x > 1.0 {
            star.norm_x = 0.0;
        }

        transform.translation.x = (star.norm_x - 0.5) * world_w;
        transform.translation.y = (star.norm_y - 0.5) * world_h;
    }
}

fn random_star_color(rng: &mut impl Rng, layer: u8) -> Color {
    let alpha = LAYER_ALPHAS[layer as usize];

    match rng.random_range(0..10) {
        0..=5 => Color::srgba(0.9, 0.93, 1.0, alpha),
        6..=7 => Color::srgba(0.75, 0.82, 1.0, alpha),
        8 => Color::srgba(1.0, 0.95, 0.88, alpha),
        _ => Color::srgba(0.7, 0.75, 0.95, alpha),
    }
}
