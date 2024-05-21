use std::time::Duration;
use bevy::prelude::*;

#[derive(Resource)]
struct BeatTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, beat)
        .add_systems(Startup, greet)
        .run();
}

fn greet(
    mut commands: Commands,
) {
    println!("Hello!");
    commands.insert_resource(BeatTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)));
}

fn beat (
    mut beat_timer: ResMut<BeatTimer>,
    time: Res<Time>
) {
    if beat_timer.0.tick(time.delta()).just_finished() {
        println!("Beat");
    }
}