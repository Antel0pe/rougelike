use specs::prelude::*;

use crate::{Position, BlocksTile};
use crate::Map;

pub struct MapIndexingSystem{ }

impl<'a> System<'a> for MapIndexingSystem{
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>,
                        Entities<'a>,);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blocks_tile, entities) = data;

        map.populated_blocked_tiles();
        map.clear_tile_content();


        for (pos, entity) in (&position, &entities).join(){
            let idx = map.xy_idx(pos.x, pos.y);

            // we only want to note an entity blocks a tile if it has the BlocksTile component
            if let Some(_) = blocks_tile.get(entity){
                map.blocked_tiles[idx] = true;
            }

            // even if an entity does not have the BlocksTile component, we want to note that they are in this idx
            map.tile_content[idx].push(entity);
        }
    }
}