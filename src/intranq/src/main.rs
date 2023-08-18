use bevy::prelude::*;
use ui::transport::TransportPlugin;
use ui::camera::CameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins((DefaultPlugins, TransportPlugin, CameraPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}