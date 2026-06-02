use crate::core::window::{CursorType, CycleCursor};
use crate::managers::asset_manager::Assets;
use crate::managers::state_manager::State;
use crate::utilities::animations::ease_in_back;
use crate::utilities::constants::{
    BUTTON_ACTION_COLOR, BUTTON_COLOR, FONT_SIZE, TITLE_HEIGHT, TITLE_WIDTH,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MainMenuScenePlugin;
impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::MainMenu), spawn_main_menu)
            .add_systems(Update, animate_title.run_if(in_state(State::MainMenu)))
            .add_systems(Update, button_system.run_if(in_state(State::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenuTitle {
    should_animate: bool,
    start_y: Option<f32>,
    time: Timer,
}

#[derive(Component)]
struct MainMenuButtons;

#[derive(Component, Debug)]
enum MainMenuButton {
    Start,
    Quit,
}

#[derive(Event)]
pub struct AnimatingTitle;

#[derive(Event)]
pub struct ButtonHover;

#[derive(Event)]
pub struct ButtonClick;

fn spawn_main_menu(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        ImageNode::new(assets.logo.clone()),
        Node {
            width: px(TITLE_WIDTH),
            height: px(TITLE_HEIGHT),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        MainMenuTitle {
            should_animate: false,
            time: Timer::from_seconds(0.6, TimerMode::Once),
            start_y: None,
        },
    ));
    commands
        .spawn((
            MainMenuButtons,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                padding: UiRect::px(50., 0., 0., 50.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                MainMenuButton::Start,
                Button,
                Text::new("Start Game"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                BUTTON_COLOR,
            ));
            parent.spawn((
                MainMenuButton::Quit,
                Button,
                Text::new("Quit"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                BUTTON_COLOR,
            ));
        });
}

fn animate_title(
    mut commands: Commands,
    mut title: Single<(Entity, &mut UiTransform, &mut MainMenuTitle)>,
    mut next_state: ResMut<NextState<State>>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let (entity, ref mut transform, ref mut t) = *title;
    if !t.should_animate {
        return;
    }

    let Val::Px(y) = transform.translation.y else {
        return;
    };

    if t.start_y.is_none() {
        t.start_y = Some(y);
    }
    let start_y = t.start_y.unwrap();
    let end_y = -window.height() / 2. - TITLE_HEIGHT;

    let tt = t.time.tick(time.delta());
    let progress = (tt.elapsed_secs() / tt.duration().as_secs_f32()).clamp(0., 1.);
    let eased = ease_in_back(progress, None);

    transform.translation.y = Val::Px(start_y + (end_y - start_y) * eased);

    if tt.just_finished() {
        commands.entity(entity).despawn();
        next_state.set(State::Playing);
    }
}

fn button_system(
    mut commands: Commands,
    mut title: Single<&mut MainMenuTitle>,
    buttons: Single<Entity, With<MainMenuButtons>>,
    interaction_query: Query<(&Interaction, &MainMenuButton, &mut TextColor), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button, mut text_color) in interaction_query {
        if *interaction == Interaction::None {
            *text_color = BUTTON_COLOR;
            commands.trigger(CycleCursor {
                type_: CursorType::Red,
            });
            continue;
        }
        *text_color = BUTTON_ACTION_COLOR;
        commands.trigger(CycleCursor {
            type_: CursorType::Purple,
        });
        if *interaction == Interaction::Hovered {
            commands.trigger(ButtonHover);
            continue;
        }
        if *interaction == Interaction::Pressed {
            commands.trigger(ButtonClick);
            match button {
                MainMenuButton::Start => {
                    title.should_animate = true;
                    commands.entity(*buttons).despawn();
                    commands.trigger(AnimatingTitle);
                }
                MainMenuButton::Quit => {
                    exit.write(AppExit::Success);
                }
            }
            continue;
        }
    }
}
