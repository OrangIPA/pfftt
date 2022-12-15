use bevy::prelude::{Deref, DerefMut};

#[derive(Clone, Copy)]
pub enum Block {
    Air,
    Dirt,
}

#[derive(Deref, DerefMut)]
pub struct Chunk([[Block; 64]; 16]);

impl Chunk {
    pub fn empty_chunk() -> Self {
        Chunk([[Block::Air;64]; 16])
    }
    
    pub fn fill_bottom_chunk(block: Block, depth: usize) -> Self {
        let mut chunk = Chunk::empty_chunk();
        for i in 0..16 {
            for j in 0..depth {
                chunk[i][j] = block;
            }
        }
        chunk
    }
}
