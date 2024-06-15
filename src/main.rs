use bevy::prelude::*;
use plugins::*;

mod plugins;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default ,Debug, States)]
enum GameState{
    #[default]
    Intro,
    Menu,
    GameSetup,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            intro::plugin,
            menu::plugin,
            game_setup::plugin,
        ))
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .run();
}

fn setup(
    mut commands: Commands
){
    commands.spawn(Camera2dBundle::default());
}