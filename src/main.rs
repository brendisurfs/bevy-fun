mod player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    let mut app = App::new();
    // plugo
    app.add_plugins(DefaultPlugins);

    // startup systems
    app.add_startup_system(spawn_camera)
        .add_startup_system(player::spawn_player);

    // systems
    app.add_system(player::player_movement)
        .add_system(player::animate_sprite)
        .add_system(player::confine_player_movement);

    // runo
    app.run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().expect("no window found");

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            1000.0,
        ),
        ..Default::default()
    });
}
