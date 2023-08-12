use bevy::prelude::*;

pub struct TransportPlugin;

impl Plugin for TransportPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_transport);
    }
}

fn spawn_transport(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle 
            {
                mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
                material: materials.add(Color::rgb(1.0, 0.4, 0.3).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0)),
                ..Default::default()
            }
        );
}