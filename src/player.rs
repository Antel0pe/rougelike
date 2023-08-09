use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
use crate::map::*;
use rltk::{Rltk, VirtualKeyCode, Tile,};
use super::*;

#[derive(Component, Debug)]
pub struct Player{ }

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World){
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let mut fov = world.write_storage::<FOV>();

    let map = world.fetch::<Map>();

    for (_player, pos, fov) in (&mut players, &mut positions, &mut fov).join(){
        // this never returns an out of bounds check because there is a wall around the border
        // the max possible value of pos.x is 78 and pos.y 48
        let destination_map_idx = map.xy_idx(pos.x+delta_x, pos.y-delta_y);

        if map.tiles[destination_map_idx] != TileType::Wall{
            // neat way to do create valid bounds for min and max
            pos.x = min(79, max(0, pos.x+delta_x));
            pos.y = min(49, max(0, pos.y-delta_y));

            // player moved so fov changed
            fov.needs_update = true;
        }
    }
}

pub fn player_input(game_state: &mut State, context: &mut Rltk){
    match context.key {
        None => {},
        Some(key) => match key {
            VirtualKeyCode::A |
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut game_state.world),

            VirtualKeyCode::W |
            VirtualKeyCode::Up => try_move_player(0, 1, &mut game_state.world),

            VirtualKeyCode::D |
            VirtualKeyCode::Right => try_move_player(1, 0, &mut game_state.world),

            VirtualKeyCode::S |
            VirtualKeyCode::Down => try_move_player(0, -1, &mut game_state.world),

            _ => {},
        },
    }
}