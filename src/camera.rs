use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component)]
pub struct TacticsGameCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        TacticsGameCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
