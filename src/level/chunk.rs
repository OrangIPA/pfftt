use bevy::prelude::{Deref, DerefMut};
use crate::level::block::BlockType;

#[derive(Deref, DerefMut, Clone)]
pub struct Chunk(pub [[BlockType; 64]; 16]);

impl Chunk {
    pub fn empty_chunk() -> Self {
        Chunk([[BlockType::Air;64]; 16])
    }
    
    pub fn fill_bottom_chunk(block: BlockType, depth: usize) -> Self {
        let mut chunk = Chunk::empty_chunk();
        for i in 0..16 {
            for j in 0..depth {
                chunk[i][j] = block;
            }
        }
        chunk
    }
}
