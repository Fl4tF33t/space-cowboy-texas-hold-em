use bevy::prelude::*;
use table::prelude::*;

mod table;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TablePlugin)
        .add_systems(PreUpdate, read_input)
        .run();
}

fn read_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    id: Res<TableIdGenerator>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let player = commands.spawn_empty().id();

        commands.trigger(RequestJoinTable {
            player,
            table_id: id.current_id()
        });
    }
}
