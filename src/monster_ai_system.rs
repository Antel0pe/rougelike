use super::*;
use rltk::console;

pub struct MonsterAI{ }

impl<'a> System<'a> for MonsterAI{
    type SystemData = ( ReadStorage<'a, Monster>,
                        WriteStorage<'a, FOV>,
                        ReadStorage<'a, Name>,
                        ReadExpect<'a, Point>,
                        WriteExpect<'a, Map>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (monster, mut fov, name, player_position, mut map, mut position) = data;

        for(_monster, fov, name, pos) in (&monster, &mut fov, &name, &mut position).join(){
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point { x: pos.x, y: pos.y }, *player_position);
            if distance < 1.5{
                console::log(format!("{} says GRRRR!", name.name));
                return;
            }

            
            if fov.visible_tiles.contains(&*player_position){
                let monster_path_to_player = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y),
                    map.xy_idx(player_position.x, player_position.y),
                    &mut *map);

                if monster_path_to_player.success && monster_path_to_player.steps.len() > 1 {
                    pos.x = monster_path_to_player.steps[1] as i32 % map.width;
                    pos.y = monster_path_to_player.steps[1] as i32 / map.width;
                    fov.needs_update = true;
                }
            }
        }
    }
}