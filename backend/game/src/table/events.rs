use bevy::prelude::*;

#[derive(Event)]
pub struct RequestJoinTable {
    pub player: Entity,
    pub table_id: u32,
}

#[derive(Event)]
pub struct RequestLeaveTable {
    pub player: Entity,
    pub table_id: u32,
}