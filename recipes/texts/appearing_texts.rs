use std::time::Duration;

use bevy::{prelude::*, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct FullText(String);

#[derive(Component)]
struct TextAppearingTimer(Timer);

#[derive(Bundle)]
struct AppearingText {
    full_text: FullText,
    timer: TextAppearingTimer,
    #[bundle]
    text_bundle: TextBundle,
}

#[derive(Component)]
struct DialogUI;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn_text(&mut commands, asset_server, "Hello, world!".to_string());
}

fn spawn_text(commands: &mut Commands, asset_server: Res<AssetServer>, text: String) -> Entity {
    let node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(30.0)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        Name::new("dialog"),
        DialogUI,
    );

    let text = AppearingText {
        full_text: FullText(text),
        timer: TextAppearingTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)),
        text_bundle: TextBundle::from_section(
            text.clone(),
            TextStyle {
                font: asset_server.load("fonts/pointfree.ttf"),
                font_size: 30.0,
                color: Color::BLACK,
            },
        ),
    };

    commands
        .spawn(node)
        .with_children(|commands| {
            commands.spawn(text);
        })
        .id()
}
