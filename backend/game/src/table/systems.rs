use bevy::prelude::*;
use super::resources::*;
use super::components::*;
use super::events::*;

pub fn startup(
    mut commands: Commands,
    mut id_generator: ResMut<TableIdGenerator>,
    mut table_registry: ResMut<TableRegistry>
) {
    const MAX_PLAYERS: usize = 6;
    let player = create_table(&mut commands, &mut id_generator, &mut table_registry, MAX_PLAYERS);

    commands.trigger(RequestJoinTable {
        player,
        table_id: id_generator.current_id()
    });
    commands.trigger(RequestLeaveTable {
        player,
        table_id: id_generator.current_id()
    });
}

pub fn handle_join_table(
    trigger: On<RequestJoinTable>,
    table_registry: Res<TableRegistry>,
    mut query: Query<&mut Seats>,
) {
    let event = trigger.event();
    let entity = event.player;
    let table_id = event.table_id;

    let Some(table) = table_registry.get(&table_id) else {
        println!("Table {:?} does not exist.", table_id);
        return;
    };

    let Ok(mut seats) = query.get_mut(*table) else {
        println!("Table entity is invalid.");
        return;
    };

    match seats.seat_player(entity) {
        Ok(_) => {
            println!("Player {:?} seated.", entity);
        }
        Err(e) => {
            println!("Could not seat player: {}", e);
        }
    }
}

pub fn handle_leave_table(
    trigger: On<RequestLeaveTable>,
    table_registry: Res<TableRegistry>,
    mut query: Query<&mut Seats>,
) {
    let event = trigger.event();
    let entity = event.player;
    let table_id = event.table_id; 

    let Some(table) = table_registry.get(&table_id) else {
        println!("Table {:?} does not exist.", table_id);
        return;
    };

    let Ok(mut seats) = query.get_mut(*table) else {
        println!("Table entity is invalid.");
        return;
    };

    match seats.unseat_player(entity) {
        Ok(_) => {
            println!("Player {:?} removed from the table.", entity);
        }
        Err(e) => {
            println!("Failed to remove player {:?} from the table: {}", entity, e);
            println!("Moving on to the next table if available...");            
        }
    }
}

fn create_table(
    commands: &mut Commands,
    id_generator: &mut TableIdGenerator,
    table_registry: &mut TableRegistry,
    max_players: usize,
) -> Entity {
    let id = id_generator.next_id();

    let entity = commands.spawn((
        Table::new(id),
        Seats::new(max_players),
    )).id();
    table_registry.insert(id,entity);

    println!("Table created with ID: {}, Max Players: {}", id, max_players);
    entity
}