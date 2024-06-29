use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub unit_blue_mesh: Handle<Mesh>,
    pub unit_blue_material: Handle<StandardMaterial>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    *scene_assets = SceneAssets {
        unit_blue_mesh: meshes.add(Capsule3d::new(0.15f32, 2.0f32)),
        unit_blue_material: materials.add(Color::rgb_u8(0, 0, 200)),
    }
}
