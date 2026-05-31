use bevy::prelude::*;

pub struct StateManagerPlugin;
impl Plugin for StateManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<State>()
            .add_systems(PostStartup, confirmed_init);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum State {
    #[default]
    Initializing,
    MainMenu,
    Loading,
    Playing,
    Paused,
    GameOver,
}

fn confirmed_init(mut next_state: ResMut<NextState<State>>) {
    info!("Post startup state transition");
    next_state.set(State::MainMenu);
}
