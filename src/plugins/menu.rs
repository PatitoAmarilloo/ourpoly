const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Menu), setup)
        .add_systems(Update, (start_button, exit_button).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), clear);
}

fn setup(
    mut cmd: Commands,
){
        cmd
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        },RootMarker))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Ourpoly",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ));
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default(),
                    ..default()
                }, StartButtonMarker))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start?",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

                parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    image: UiImage::default(),
                    ..default()
                }, ExitButtonMarker))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit?",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

fn clear(
    mut commands: Commands,
    query: Query<Entity, With<RootMarker>>
){
    commands.entity(query.single()).despawn_recursive();
}

fn start_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>, With<StartButtonMarker>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut style, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Start!".to_string();
                style.0 = PRESSED_BUTTON;
                border_color.0 = Color::RED;
                next_state.set(GameState::GameSetup);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Start".to_string();
                style.0 = HOVERED_BUTTON;
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Start?".to_string();
                style.0 = NORMAL_BUTTON;
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn exit_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>, With<ExitButtonMarker>),
    >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    windows: Query<Entity, With<Window>>,
) {
    for (interaction, mut style, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                println!("wut");
                text.sections[0].value = "Exit!".to_string();
                style.0 = PRESSED_BUTTON;
                border_color.0 = Color::RED;
                for window in windows.iter() {
                    commands.entity(window).despawn()
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Exit".to_string();
                style.0 = HOVERED_BUTTON;
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Exit?".to_string();
                style.0 = NORMAL_BUTTON;
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[derive(Component)]
struct StartButtonMarker;

#[derive(Component)]
struct RootMarker;

#[derive(Component)]
struct ExitButtonMarker;