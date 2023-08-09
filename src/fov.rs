use crate::Player;

use super::{Position, FOV, Map};
use rltk::{field_of_view, Point};
use specs::{prelude::*, rayon::vec};

pub struct FovSystem { }

impl<'a> System<'a> for FovSystem{
    // ReadExpect because not getting map is a failure? Are not getting the others possible? 
    type SystemData = ( ReadStorage<'a, Player>,
                        Entities<'a>,
                        WriteExpect<'a, Map>,
                        WriteStorage<'a, FOV>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, system_data: Self::SystemData) {
        let (player, entities, mut map, mut fov, position) = system_data;

        for (ent, fov, pos) in (&entities, &mut fov, &position).join(){
            if !fov.needs_update{
                continue;
            } 

            fov.visible_tiles.clear();

            // &*map is dereferencing map THEN referencing it
            // https://stackoverflow.com/questions/41273041/what-does-combined-together-do-in-rust
            // ^ mentions that this might be because types can dereference to a DIFFERENT type than the original
            // is *map dereferencing to Algorithm2D which is a trait implemented for map and input type of this func?
            // tutorial says - "slightly convoluted "dereference, then get a reference" to unwrap Map from the ECS."
            fov.visible_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);

            // Retain only tiles that are within the bounds of the map
            fov.visible_tiles.retain(|tile| tile.x >= 0 && tile.x < map.width && tile.y >= 0 && tile.y < map.height);

            if let Some(_p) = player.get(ent){
                // since fov needs updating, the currently visible tiles have changed
                map.currently_visible_tiles = vec![false; (map.height*map.width) as usize];

                for tile in fov.visible_tiles.iter(){
                    let idx = map.xy_idx(tile.x, tile.y);
                    map.revealed_tiles[idx] = true;
                    map.currently_visible_tiles[idx] = true;
                }
            }

            fov.needs_update = false;
        }
    }


}
