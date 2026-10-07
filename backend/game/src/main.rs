use bevy::prelude::*;

use table::prelude::*;

mod table;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TablePlugin)
        .run();
}

fn read_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // commands.trigger(RequestJoinTable {
            
        // });
        println!("Space key was just pressed!");
    }
}