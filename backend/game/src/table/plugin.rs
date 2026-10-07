use bevy::prelude::*;
use super::systems::*;
use super::resources::*;
pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TableRegistry>()
            .init_resource::<TableIdGenerator>()
            .add_systems(Startup, startup)
            .add_observer(handle_join_table)
            .add_observer(handle_leave_table);
    }
}