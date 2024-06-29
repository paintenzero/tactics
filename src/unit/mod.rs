use bevy::prelude::*;
pub mod events;
use crate::asset_loader::SceneAssets;
use crate::unit::events::UnitSpawnEvent;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, unit_spawn_event_reader)
            .add_event::<UnitSpawnEvent>();
    }
}

#[derive(Component, Debug)]
pub struct TacticsUnit;

#[derive(Bundle)]
struct UnitBundle {
    unit: TacticsUnit,
    model: PbrBundle,
}

fn unit_spawn_event_reader(
    mut commands: Commands,
    mut event_reader: EventReader<UnitSpawnEvent>,
    scene_assets: Res<SceneAssets>,
) {
    for &UnitSpawnEvent { transform, parent } in event_reader.read() {
        let unit_entity = commands
            .spawn(UnitBundle {
                unit: TacticsUnit,
                model: PbrBundle {
                    mesh: scene_assets.unit_blue_mesh.clone(),
                    material: scene_assets.unit_blue_material.clone(),
                    transform: transform,
                    ..default()
                },
            })
            .id();
        commands.entity(parent).add_child(unit_entity);
    }
}
