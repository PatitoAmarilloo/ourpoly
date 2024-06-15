use bevy::prelude::*;

use crate::GameState;

#[derive(Resource)]
struct LogoTimer(Timer);

#[derive(Resource)]
struct SkipTimer(Timer);

#[derive(Component)]
struct TextMarker;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Intro), setup)
        .add_systems(Update, (update,skip).run_if(in_state(GameState::Intro)))
        .add_systems(OnExit(GameState::Intro), clear);
}

fn setup(
    mut cmd: Commands
){
    cmd.insert_resource(LogoTimer(Timer::from_seconds(2.5, TimerMode::Once)));
    cmd.insert_resource(SkipTimer(Timer::from_seconds(1.5, TimerMode::Once)));

    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            TextBundle::from_section("Dionnor Presents", TextStyle{
                font_size: 120.,
                ..default()
            })
            .with_text_justify(JustifyText::Center),
            TextMarker
        ));
    });
}

fn clear(
    mut cmd: Commands,
    mut text: Query<Entity, With<Text>>,
){
    cmd.remove_resource::<LogoTimer>();
    cmd.remove_resource::<SkipTimer>();

    cmd.entity(text.single_mut()).despawn();
}

fn update(
    mut next_state: ResMut<NextState<GameState>>,
    mut logo: Local<u8>,
    mut timer: ResMut<LogoTimer>,
    time: Res<Time>,
    mut text: Query<&mut Text, With<TextMarker>>
){
    if timer.0.tick(time.delta()).finished() {
        *logo = *logo + 1;
        timer.0.reset();
        text.single_mut().sections[0].value = match *logo {
            1 =>"Ourpoly",
            2 =>"Made in Rust",
            3 =>"Powered by Bevy",
            _ =>{
                next_state.set(GameState::Menu);
                return;
            }
        }.into();
        
    }
}

fn skip(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<SkipTimer>,
    time: Res<Time>
){
    if keyboard.pressed(KeyCode::Escape) {
        println!("skipping");
        if timer.0.tick(time.delta()).finished() {
            next_state.set(GameState::Menu);
            println!("skipped");
        }
    }else if keyboard.just_released(KeyCode::Escape) {
        timer.0.reset();
    }
}