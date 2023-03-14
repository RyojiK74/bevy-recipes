use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_window)
        .run();
}

fn init_window(mut window: Query<&mut Window>) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };

    window.title = "NEW WINDOW IS OPENING".to_string();
    window.resolution = WindowResolution::new(1980., 1200.);
}
