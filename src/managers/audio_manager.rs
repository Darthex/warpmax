use crate::managers::asset_manager::Assets;
use crate::managers::state_manager::State as S;
use crate::utilities::constants::SOUNDTRACK_FADE_TIME;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

pub struct AudioManagerPlugin;
impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, (change_track, fade_in, fade_out))
            .add_observer(play_sfx);
    }
}

#[derive(Resource)]
struct SoundtrackPlayer {
    track_list: Vec<Handle<AudioSource>>,
}

impl SoundtrackPlayer {
    fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
        Self { track_list }
    }
}

#[derive(Component)]
struct FadeIn {
    timer: Timer,
}

#[derive(Component)]
struct FadeOut {
    timer: Timer,
}

impl FadeIn {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(SOUNDTRACK_FADE_TIME, TimerMode::Once),
        }
    }
}

impl FadeOut {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(SOUNDTRACK_FADE_TIME, TimerMode::Once),
        }
    }
}

pub enum Sfx {
    Hover,
    Click,
    Whoosh,
}

#[derive(Event)]
pub struct PlaySfx(pub Sfx);

fn setup(mut commands: Commands, assets: Res<Assets>) {
    let track_1 = assets.main_menu_bg.clone();
    let track_2 = assets.game_bg.clone();
    let track_list = vec![track_1.clone(), track_2];
    commands.insert_resource(SoundtrackPlayer::new(track_list));
    commands.spawn((AudioPlayer::new(track_1), PlaybackSettings::LOOP));
}

fn change_track(
    mut commands: Commands,
    soundtrack_player: Res<SoundtrackPlayer>,
    soundtrack: Query<Entity, (With<AudioSink>, Without<FadeOut>)>,
    game_state: Res<State<S>>,
) {
    if game_state.is_changed() {
        for entity in soundtrack.iter() {
            commands.entity(entity).insert(FadeOut::new());
        }
        let next_track = match game_state.get() {
            S::MainMenu => soundtrack_player.track_list[0].clone(),
            S::Playing => soundtrack_player.track_list[1].clone(),
            _ => return,
        };
        commands.spawn((
            AudioPlayer(next_track),
            PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::SILENT,
                ..default()
            },
            FadeIn::new(),
        ));
    }
}

fn fade_in(
    mut commands: Commands,
    mut query: Query<(&mut AudioSink, &mut FadeIn, Entity)>,
    time: Res<Time>,
) {
    for (mut sink, mut fade, entity) in &mut query {
        fade.timer.tick(time.delta());
        let progress = fade.timer.elapsed_secs() / SOUNDTRACK_FADE_TIME;
        sink.set_volume(Volume::Linear(progress.clamp(0., 1.)));
        if fade.timer.just_finished() {
            sink.set_volume(Volume::Linear(1.0));
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn fade_out(
    mut commands: Commands,
    mut query: Query<(&mut AudioSink, &mut FadeOut, Entity)>,
    time: Res<Time>,
) {
    for (mut sink, mut fade, entity) in &mut query {
        fade.timer.tick(time.delta());
        let progress = fade.timer.elapsed_secs() / SOUNDTRACK_FADE_TIME;
        sink.set_volume(Volume::Linear((1.0 - progress).clamp(0., 1.)));
        if fade.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn play_sfx(sfx: On<PlaySfx>, mut commands: Commands, assets: Res<Assets>) {
    let _sfx = match sfx.0 {
        Sfx::Hover => assets.hover_sfx.clone(),
        Sfx::Click => assets.click_sfx.clone(),
        Sfx::Whoosh => assets.whoosh_sfx.clone(),
    };
    commands.spawn((AudioPlayer::new(_sfx), PlaybackSettings::DESPAWN));
}
