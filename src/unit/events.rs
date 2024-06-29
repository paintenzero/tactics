use bevy::{
    prelude::{Entity, Event},
    transform::components::Transform,
};

#[derive(Event, Debug)]
pub struct UnitSpawnEvent {
    pub parent: Entity,
    pub transform: Transform,
}
