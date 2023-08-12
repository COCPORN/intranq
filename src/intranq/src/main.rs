use bevy::prelude::*;
use ui::transport::TransportPlugin;
use ui::camera::CameraPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TransportPlugin, CameraPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}