use super::*;
use rltk::console;

pub struct MonsterAI{ }

impl<'a> System<'a> for MonsterAI{
    type SystemData = ( ReadStorage<'a, Monster>,
                        WriteStorage<'a, FOV>,
                        ReadExpect<'a, Point>,
                        WriteExpect<'a, Map>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>,
                        Entities<'a>, // gets all entities
                        ReadExpect<'a, Entity>, // gets player entity resource
                        ReadExpect<'a, RunState>,
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (monsters,
             mut fov,
             player_position,
             mut map,
             mut position,
             mut wants_to_melee,
             entities,
             player_entity,
             run_state,
            ) = data;

        if *run_state != RunState::MonsterTurn{ return; }

        for(_monster, fov, pos, entity) in (&monsters, &mut fov, &mut position, &entities).join(){
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point { x: pos.x, y: pos.y }, *player_position);
            if distance < 1.5{
                wants_to_melee.insert(entity, WantsToMelee { target: *player_entity })
                    .expect("Could not add wants to melee component to monster with target player.");
            }

            
            if fov.visible_tiles.contains(&*player_position){
                let monster_path_to_player = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y),
                    map.xy_idx(player_position.x, player_position.y),
                    &mut *map);

                console::log(&format!("Success: {}, Steps: {}", monster_path_to_player.success, monster_path_to_player.steps.len()));
                if monster_path_to_player.success && monster_path_to_player.steps.len() > 1 {
                    // moving out of previous tile, not blocking it anymore
                    let prev_idx = map.xy_idx(pos.x, pos.y);
                    map.blocked_tiles[prev_idx] = false;

                    pos.x = monster_path_to_player.steps[1] as i32 % map.width;
                    pos.y = monster_path_to_player.steps[1] as i32 / map.width;

                    // now blocking new tile
                    let next_idx = map.xy_idx(pos.x, pos.y);
                    map.blocked_tiles[next_idx] = true;

                    fov.needs_update = true;
                }
            }
        }
    }
}