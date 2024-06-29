use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TacticsSystemSet {
    UserInput,
    UnitsAction,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (TacticsSystemSet::UserInput, TacticsSystemSet::UnitsAction).chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(TacticsSystemSet::UserInput)
                .before(TacticsSystemSet::UnitsAction),
        );
    }
}
