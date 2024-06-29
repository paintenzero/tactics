use std::cell;

use crate::{camera::TacticsGameCamera, schedule::TacticsSystemSet, unit::events::UnitSpawnEvent};
use bevy::input::keyboard::{self, KeyboardInput};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const GROUND_HEIGHT: f32 = 0.2f32;
const GROUND_SIZE: (f32, f32) = (5.0f32, 5.0f32);
const GRID_SIZE: (f32, f32) = (10.0f32, 10.0f32);
pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ground).add_systems(
            Update,
            (handle_mouse_click, handle_keyboard_input).in_set(TacticsSystemSet::UserInput),
        );
    }
}

#[derive(Component)]
pub struct BattleGround;

#[derive(Bundle)]
struct GroundBundle {
    ground: BattleGround,
    model: PbrBundle,
}

fn create_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(GroundBundle {
        ground: BattleGround,
        model: PbrBundle {
            mesh: meshes.add(Cuboid::new(GROUND_SIZE.0, GROUND_HEIGHT, GROUND_SIZE.1)),
            material: materials.add(Color::rgb_u8(0, 200, 0)),
            transform: Transform::from_translation(Vec3::new(0.0f32, -GROUND_HEIGHT / 2.0, 0.0f32)),
            ..default()
        },
    });
}

fn handle_mouse_click(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<TacticsGameCamera>>,
    ground_query: Query<(Entity, &GlobalTransform), With<BattleGround>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut unit_spawn_event_writer: EventWriter<UnitSpawnEvent>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let (ground_entity, ground_transform) = ground_query.single();
    let ground_up = ground_transform.up();
    let window = window_query.single();
    // Get cursor position if it is in main window
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let plane = Plane3d::new(ground_up);
    let plane_origin = ground_transform.translation();
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return;
    };
    let ground_point = ray.get_point(distance);
    let local_point = ground_point - ground_transform.translation()
        + Vec3::new(GROUND_SIZE.0 / 2.0f32, 0.0f32, GROUND_SIZE.1 / 2.0f32);
    if local_point.x > GROUND_SIZE.0
        || local_point.x < 0.0f32
        || local_point.z > GROUND_SIZE.1
        || local_point.z < 0.0f32
    {
        return;
    }
    gizmos.circle(
        ground_point + ground_up * (GROUND_HEIGHT / 2.0 + 0.01),
        Direction3d::new_unchecked(ground_up), // Up vector is already normalized.
        0.2,
        Color::WHITE,
    );
    if mouse_buttons.just_pressed(MouseButton::Left) {
        // Lets assume we have 10x10 grid. Calculate the cell we clicked on
        let cell_size: (f32, f32) = (GROUND_SIZE.0 / GRID_SIZE.0, GROUND_SIZE.1 / GRID_SIZE.1);

        let col = (local_point.x / cell_size.0).floor();
        let row = (local_point.z / cell_size.1).floor();

        let spawn_x = ground_transform.translation().x - GROUND_SIZE.0 * 0.5f32
            + (col + 0.5f32) * (cell_size.0);
        let spawn_z = ground_transform.translation().z - GROUND_SIZE.1 * 0.5f32
            + (row + 0.5f32) * cell_size.1;
        let spawn_point: Vec3 = Vec3::new(spawn_x, 0.0f32, spawn_z);

        unit_spawn_event_writer.send(UnitSpawnEvent {
            transform: Transform::from_translation(spawn_point),
            parent: ground_entity,
        });
    }
}

fn handle_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Do spawn all!");
    }
}
