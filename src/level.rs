mod chunk;
mod block;

use chunk::Chunk;
use bevy::prelude::*;

#[allow(unused)]
use crate::SCALE;

use self::block::BlockType;

#[derive(Resource, Deref, DerefMut)]
pub struct PlayerSpawned(pub bool);
#[derive(Resource, Deref, DerefMut)]
pub struct SpawnPosition(pub Option<Vec2>);

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
    }
}

fn init(
    spawn_pos: ResMut<SpawnPosition>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let level = Level::new_fill_bottom(10, 20, BlockType::Dirt);
    level.load_level(spawn_pos, &mut commands, &asset_server, &mut texture_atlases)
}

pub struct Level {
    chunks: Vec<Chunk>,
    spawn_pos: Vec2,
}

impl Level {
    pub fn load_level(
        &self,
        mut spawn_pos: ResMut<SpawnPosition>,
        mut commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>
    ) {
        **spawn_pos = Some(self.spawn_pos);
        for (chunk_no, chunk) in self.chunks.iter().enumerate() {
            for (column, i) in (**chunk).iter().enumerate() {
                println!("kolom: {}\n", column);
                for (row, j) in i.iter().enumerate() {
                    j.spawn((column as f32 + (16. * chunk_no as f32), row as f32),
                        self.get_surrounding_block((column + (16 * chunk_no), row)),
                        &mut commands,
                        &mut texture_atlases,
                        &asset_server);
                }
            }
        }
    }

    pub fn get_surrounding_block(&self, mut pos: (usize, usize)) -> [Option<BlockType>; 4] {
        let chunk_no = pos.0/ 16;
        pos.0 = pos.0 % 16;
        // print!("chunk no: {}\t", chunk_no);
        let mut ret: [Option<BlockType>; 4] = default();
        let chunk = self.chunks.get(chunk_no).unwrap();
        let (left_chunk, right_chunk) = self.neighboring_chunk(chunk_no);
        ret[0] = match pos.1 {
            63 => None,
            _ => Some(chunk[pos.0][pos.1 + 1]),
        };
        ret[1] = match pos.0 {
            15 => match right_chunk {
                None => None,
                Some(s) => Some(s.clone()[0][pos.1])
            },
            _ => Some(chunk[pos.0 + 1][pos.1])
        };
        ret[2] = match pos.1 {
            0 => None,
            _ => Some(chunk[pos.0][pos.1 - 1])
        };
        ret[3] = match pos.0 {
            0 => match left_chunk {
                None => None,
                Some(s) => Some(s.clone()[15][pos.1])
            },
            _ => Some(chunk[pos.0 - 1][pos.1])
        };
        ret
    }
    
    pub fn new_fill_bottom(chunk_len: usize, depth: usize , block: BlockType) -> Self {
        Level {
            chunks: {
                let val = Chunk::fill_bottom_chunk(block, depth);
                let mut ret = vec![val.clone()];
                for i in 0..chunk_len {
                    if i == 0 {continue;}
                    ret.push(val.clone());
                } 
                ret
            },
            spawn_pos: Vec2::new(1., (depth + 5) as f32),
        }
    }
    
    fn neighboring_chunk(&self, chunk_no: usize) -> (Option<&Chunk>, Option<&Chunk>) {
        if chunk_no == 0 {return (None, self.chunks.get(1));}
        (self.chunks.get(chunk_no - 1), self.chunks.get(chunk_no + 1))
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::level::Level;

    use super::block::BlockType;
    #[test]
    fn gs() {
        let level = Level::new_fill_bottom(4, 5, BlockType::Dirt);
        assert_eq!(level.get_surrounding_block((15, 1)), [Some(BlockType::Dirt), None, Some(BlockType::Dirt), Some(BlockType::Dirt)]);
    }
}