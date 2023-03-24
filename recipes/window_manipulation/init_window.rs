use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGHT: f32 = 600.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "New window!!".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .run();
}
