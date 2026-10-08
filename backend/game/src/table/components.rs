use bevy::prelude::*;

#[derive(Component)]
pub struct Table {
    id: u32,
}

impl Table {
    pub fn new(id: u32) -> Self {
        Table {
            id,
        }
    }
    // pub fn get_id(&self) -> u32 {
    //     self.id
    // }
}


#[derive(Component)]
pub struct Seats {
    pub players: Vec<Option<Entity>>,
}

impl Seats {
    pub fn new(max_players: usize) -> Self {
        Seats {
            players: vec![None; max_players],
        }
    }
    // pub fn clear(&mut self) {
    //     self.players.clear();
    // }

    pub fn seat_player(&mut self, player_entity: Entity) -> Result<(), &'static str> {
        for seat in &mut self.players {
            if seat.is_none() {
                *seat = Some(player_entity);
                return Ok(());
            }
        }
        Err("No available seats")
    }

    pub fn unseat_player(&mut self, player_entity: Entity) -> Result<(), &'static str> {
        for seat in &mut self.players {
            if *seat == Some(player_entity) {
                *seat = None;
                return Ok(());
            }
        }
        Err("Player not found in any seat")
    }
}