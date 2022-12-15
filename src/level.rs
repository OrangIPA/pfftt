mod chunk;

use chunk::{Block, Chunk};
use bevy::prelude::*;

pub struct Level {
    blocks: Vec<Chunk>,
    spawn_pos: (i32, i32),
}

impl Level {
    fn load_level(
        &self,
        mut commands: Commands,
    ) {
        
    }
}
