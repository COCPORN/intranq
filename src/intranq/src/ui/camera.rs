use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,    
) {
    commands
        .spawn(Camera3dBundle 
            {
                projection: OrthographicProjection {                    
                    scale: 3.0,
                    scaling_mode: ScalingMode::FixedVertical(5.0),
                    ..default()
                }
                .into(),
                transform: Transform::from_xyz(0.0, 0.0, 22.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            }
        );
}