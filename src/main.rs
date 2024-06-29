mod asset_loader;
mod camera;
mod ground;
mod schedule;
mod unit;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use ground::GroundPlugin;
use schedule::SchedulePlugin;
use unit::UnitPlugin;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 300.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(UnitPlugin)
        .add_plugins(CameraPlugin)
        .run()
}
