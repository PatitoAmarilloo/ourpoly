use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GameSetup), setup)
        .add_systems(Update, (update).run_if(in_state(GameState::GameSetup)))
        .add_systems(OnExit(GameState::GameSetup), clear);
}

fn setup(
    mut commands: Commands
){
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                padding: UiRect::all(Val::Px(40.)),
                ..default()
            },
            ..default()
        },RootMarker));
}

fn clear(
    mut _commands: Commands
){
    
}

fn update(
    mut _commands: Commands
){
    
}

#[derive(Component)]
struct RootMarker;

#[derive(Component)]
struct Name(String);