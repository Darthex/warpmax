use crate::managers::asset_manager::RingMaterial;
use crate::managers::audio_manager::{PlaySfx, Sfx};
use crate::managers::state_manager::State;
use crate::utilities::animations::ease_in_back;
use crate::utilities::constants::{
    ARENA_BORDER_WIDTH, ARENA_COLOR, ARENA_HEIGHT, ARENA_WIDTH, COLOR_RED, LOADING_TIMER,
};
use bevy::prelude::*;

pub struct GameScenePlugin;
impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(State::Loading),
            (setup_arena, setup_loading).chain(),
        )
        .add_systems(
            Update,
            update_ring
                .run_if(in_state(State::Loading))
                .run_if(any_with_component::<LoadingRing>),
        )
        .add_systems(OnExit(State::Loading), cleanup_loading)
        .add_systems(Update, clamp_to_arena.run_if(in_state(State::Playing)));
    }
}

#[derive(Component)]
pub struct ClampRadius(pub f32);

#[derive(Component)]
struct LoadingRing {
    timer: Timer,
    exit_timer: Option<Timer>,
}

#[derive(Component)]
struct LoadingRingText;

fn setup_arena(mut commands: Commands) {
    // Left, bottom, right, top
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_BORDER_WIDTH,
                ARENA_HEIGHT + (ARENA_BORDER_WIDTH * 2.),
            )),
            ..default()
        },
        Transform::from_xyz((-ARENA_WIDTH / 2.) - ARENA_BORDER_WIDTH, 0., 0.),
    ));
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_WIDTH + (ARENA_BORDER_WIDTH * 2.0),
                ARENA_BORDER_WIDTH,
            )),
            ..default()
        },
        Transform::from_xyz(0., (-ARENA_HEIGHT / 2.) - ARENA_BORDER_WIDTH, 0.),
    ));
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_BORDER_WIDTH,
                ARENA_HEIGHT + (ARENA_BORDER_WIDTH * 2.),
            )),
            ..default()
        },
        Transform::from_xyz((ARENA_WIDTH / 2.) + ARENA_BORDER_WIDTH, 0., 0.),
    ));
    commands.spawn((
        Sprite {
            color: ARENA_COLOR,
            custom_size: Some(Vec2::new(
                ARENA_WIDTH + (ARENA_BORDER_WIDTH * 2.),
                ARENA_BORDER_WIDTH,
            )),
            ..default()
        },
        Transform::from_xyz(0., (ARENA_HEIGHT / 2.) + ARENA_BORDER_WIDTH, 0.),
    ));
}

fn clamp_to_arena(entities: Query<(&mut Transform, &ClampRadius)>) {
    for (mut transform, clamp_radius) in entities {
        let half_w = ARENA_WIDTH / 2.0 - clamp_radius.0;
        let half_h = ARENA_HEIGHT / 2.0 - clamp_radius.0;

        transform.translation.x = transform.translation.x.clamp(-half_w, half_w);
        transform.translation.y = transform.translation.y.clamp(-half_h, half_h);
    }
}

fn setup_loading(mut commands: Commands, mut materials: ResMut<Assets<RingMaterial>>) {
    commands
        .spawn((
            LoadingRing {
                timer: Timer::from_seconds(LOADING_TIMER as f32, TimerMode::Once),
                exit_timer: None,
            },
            MaterialNode(materials.add(RingMaterial {
                progress: 1.0,
                color: LinearRgba::rgb(COLOR_RED.0, COLOR_RED.1, COLOR_RED.2),
            })),
            Node {
                width: px(120.),
                height: px(120.),
                position_type: PositionType::Absolute,
                top: Val::Percent(50.),
                left: Val::Percent(50.),
                margin: UiRect {
                    left: px(-60.),
                    top: px(-60.),
                    ..default()
                },
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: px(110.),
                    height: px(110.),
                    border: UiRect::all(px(2.)),
                    border_radius: BorderRadius::all(px(999.)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)),
            ));
            parent.spawn((
                LoadingRingText,
                Text(LOADING_TIMER.to_string()),
                TextColor(ARENA_COLOR),
            ));
        });
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingRing>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn update_ring(
    mut commands: Commands,
    mut ring: Single<(&mut LoadingRing, &MaterialNode<RingMaterial>, &mut Node)>,
    mut text: Single<&mut Text, With<LoadingRingText>>,
    mut materials: ResMut<Assets<RingMaterial>>,
    mut next_state: ResMut<NextState<State>>,
    time: Res<Time>,
) {
    let (ref mut r, m_node, ref mut ui_node) = *ring;

    if let Some(ref mut exit) = r.exit_timer {
        exit.tick(time.delta());
        let progress = exit.fraction();
        let eased = ease_in_back(progress, Some(1.0));
        let start = 50.0;
        let end = -20.0;
        ui_node.top = Val::Percent(start + (end - start) * eased);
        if exit.just_finished() {
            next_state.set(State::Playing);
        }
        return;
    }

    r.timer.tick(time.delta());

    let seconds_left = (r.timer.duration().as_secs_f32() - r.timer.elapsed_secs()).ceil() as u32;
    text.0 = seconds_left.to_string();

    if let Some(mat) = materials.get_mut(m_node.id()) {
        mat.progress = 1.0 - r.timer.fraction();
    }

    if r.timer.just_finished() {
        commands.trigger(PlaySfx(Sfx::Whoosh));
        r.exit_timer = Some(Timer::from_seconds(0.6, TimerMode::Once));
    }
}
