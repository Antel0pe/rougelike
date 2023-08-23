use specs::prelude::*;
use std::cmp::{max, min};
use rltk::{Rltk, VirtualKeyCode};
use super::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World){
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let mut fov = world.write_storage::<FOV>();
    let mut player_position = world.write_resource::<Point>();
    let combat_stats = world.read_storage::<CombatStats>();
    let mut wants_to_melee = world.write_storage::<WantsToMelee>();
    let mut has_movement_speed_modifier = world.write_storage::<HasMovementSpeedModifier>();

    let entities = world.entities();

    let map = world.fetch::<Map>();

    for (_player, pos, fov, entity) in (&mut players, &mut positions, &mut fov, &entities).join(){
        let mut modified_delta_x = delta_x;
        let mut modified_delta_y = delta_y;

        if let Some(movement_speed_modifer) = has_movement_speed_modifier.get_mut(entity){
            modified_delta_x *= movement_speed_modifer.speed_modifier;
            modified_delta_y *= movement_speed_modifer.speed_modifier;

            movement_speed_modifer.turns_used += 1;
        }

        // this never returns an out of bounds check because there is a wall around the border
        // the max possible value of pos.x is 78 and pos.y 48
        let destination_map_idx = map.xy_idx(pos.x+modified_delta_x, pos.y-modified_delta_y);

        // for each entity in the destination tile, see if they have combat stats. If they do fight them
        for potential_target in map.tile_content[destination_map_idx].iter(){
            if let Some(_target) = combat_stats.get(*potential_target){
                // add wants to melee component with target as potential target
                wants_to_melee.insert(entity, WantsToMelee { target: *potential_target })
                    .expect("Cannot add component wants_to_melee to taget");
                return; // dont want to continue to move on top of target
            }
        }


        if !map.blocked_tiles[destination_map_idx]{
            // neat way to do create valid bounds for min and max
            pos.x = min(map.width-1, max(0, pos.x+modified_delta_x));
            pos.y = min(map.height-1, max(0, pos.y-modified_delta_y));

            // update player position
            player_position.x = pos.x;
            player_position.y = pos.y;

            // player moved so fov changed
            fov.needs_update = true;
        }
    }
}

pub fn player_input(game_state: &mut State, context: &mut Rltk) -> RunState{
    match context.key {
        None => { return RunState::AwaitingInput; }, // if no key pressed, no update for game to run on
        Some(key) => match key {
            VirtualKeyCode::A |
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.world),

            VirtualKeyCode::W |
            VirtualKeyCode::Up => try_move_player(0, 1, &mut game_state.world),

            VirtualKeyCode::D |
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.world),

            VirtualKeyCode::S |
            VirtualKeyCode::Down => try_move_player(0, -1, &mut game_state.world),

            // diagonals
            VirtualKeyCode::Q => try_move_player(-1, 1, &mut game_state.world),

            VirtualKeyCode::E => try_move_player(1, 1, &mut game_state.world),

            VirtualKeyCode::C => try_move_player(1, -1, &mut game_state.world),

            VirtualKeyCode::Z => try_move_player(-1, -1, &mut game_state.world),

            // pick up item
            VirtualKeyCode::G => pickup_item(&mut game_state.world),

            VirtualKeyCode::I => return RunState::InInventory,

            VirtualKeyCode::R => return RunState::ShowDropItem,

            VirtualKeyCode::Escape => return RunState::SaveGame,

            VirtualKeyCode::M => return RunState::MainMenu { menu_selection: MainMenuSelection::NewGame },

            _ => { return RunState::AwaitingInput; }, // if irrelevant key pressed, nothing for game to update on
        },
    }

    // if player just moved, we need to run the game to update stuff
    RunState::PlayerTurn 
}


pub fn pickup_item(world: &mut World){
    let player_position = world.read_resource::<Point>();
    let player_entity = world.fetch::<Entity>();
    let mut pick_up_items = world.write_storage::<WantsToPickUpItem>();
    let positions = world.read_storage::<Position>();
    let entities = world.entities();
    let items = world.read_storage::<Item>();
    let mut gamelog = world.write_resource::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (_item, pos, entity) in (&items, &positions, &entities).join(){
        if pos.x == player_position.x && pos.y == player_position.y{
            target_item = Some(entity);
        }
    }

    match target_item{
        None => gamelog.entries.push("Nothing to pick up here...".to_string()),
        Some(item) => {
            pick_up_items.insert(item, WantsToPickUpItem { collected_by: *player_entity, item })
                .expect("Unable to pick up item by player.");
        },
    }

}