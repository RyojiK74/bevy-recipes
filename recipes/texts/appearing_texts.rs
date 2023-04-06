use std::time::Duration;

use bevy::{prelude::*, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(appeare_text)
        .run();
}

#[derive(Component)]
struct AppearingTextConfig {
    full_text: String,
    timer: Timer,
    text_index: usize,
}

#[derive(Component)]
struct FullText(String);

#[derive(Component)]
struct TextAppearingTimer(Timer);

#[derive(Bundle)]
struct AppearingText {
    appearing_text_config: AppearingTextConfig,
    #[bundle]
    text_bundle: TextBundle,
}

#[derive(Component)]
struct AppearingTextMarker;

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
        appearing_text_config: AppearingTextConfig {
            full_text: text.clone(),
            timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating),
            text_index: 1,
        },
        text_bundle: TextBundle::from_section(
            get_head_string_by_length(&text, 1),
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
            commands.spawn(text).insert(AppearingTextMarker);
        })
        .id()
}

fn appeare_text(
    mut text: Query<(&mut Text, &mut AppearingTextConfig), With<AppearingTextMarker>>,
    time: Res<Time>,
) {
    for (mut text, mut config) in text.iter_mut() {
        if config.timer.tick(time.delta()).just_finished() {
            config.text_index += 1;
            text.sections[0].value =
                get_head_string_by_length(&config.full_text, config.text_index);
        }
    }
}

fn get_head_string_by_length(text: &str, length: usize) -> String {
    let mut s = String::new();
    for (i, char) in text.chars().enumerate() {
        match i {
            i if i < length => s.push(char),
            _ => break,
        }
    }
    s
}
