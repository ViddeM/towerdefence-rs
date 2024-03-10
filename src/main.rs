use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use builder::tile_build_system;
use map::map_setup;
use ui::{button_system, setup_ui};
use wave::{move_enemies, wave_spawner, waves_setup};

pub mod builder;
pub mod map;
pub mod ui;
pub mod utils;
pub mod wave;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Territorial RS".into(),
                        resolution: (2900., 1600.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (map_setup.before(setup_ui), setup_ui, waves_setup))
        .add_systems(
            Update,
            (tile_build_system, button_system, wave_spawner, move_enemies),
        )
        .run();
}
