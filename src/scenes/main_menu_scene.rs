use crate::managers::asset_manager::Assets;
use crate::managers::state_manager::State;
use crate::utilities::constants::{BUTTON_ACTION_COLOR, BUTTON_COLOR};
use bevy::prelude::*;

pub struct MainMenuScenePlugin;
impl Plugin for MainMenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::MainMenu), spawn_main_menu)
            .add_systems(OnExit(State::MainMenu), despawn_main_menu)
            .add_systems(Update, button_system.run_if(in_state(State::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenuScreen;

#[derive(Component, Debug)]
enum MainMenuButton {
    Start,
    Quit,
}

fn spawn_main_menu(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        ImageNode::new(assets.logo.clone()),
        Node {
            width: px(600.),
            height: px(256.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
    ));
    commands
        .spawn((
            MainMenuScreen,
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
                    font_size: 32.0,
                    ..default()
                },
                BUTTON_COLOR,
            ));
            parent.spawn((
                MainMenuButton::Quit,
                Button,
                Text::new("Quit"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                BUTTON_COLOR,
            ));
        });
}

fn despawn_main_menu(mut commands: Commands, screen_entity: Single<Entity, With<MainMenuScreen>>) {
    commands.entity(*screen_entity).despawn();
}

// TODO: add pointer cursors?
fn button_system(
    mut next_state: ResMut<NextState<State>>,
    interaction_query: Query<(&Interaction, &MainMenuButton, &mut TextColor), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button, mut text_color) in interaction_query {
        if *interaction == Interaction::None {
            *text_color = BUTTON_COLOR;
            continue;
        }
        *text_color = BUTTON_ACTION_COLOR;
        if *interaction == Interaction::Pressed {
            match button {
                MainMenuButton::Start => {
                    next_state.set(State::Loading);
                }
                MainMenuButton::Quit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}
