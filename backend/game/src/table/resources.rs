use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct TableRegistry(pub HashMap<u32, Entity>);

#[derive(Resource, Default)]
pub struct TableIdGenerator(u32);

impl TableIdGenerator {
    pub fn next_id(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
    pub fn current_id(&self) -> u32 {
        self.0
    }
}