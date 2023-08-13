use rltk::{ RGB, Rltk, RandomNumberGenerator, Algorithm2D, Point, BaseMap, FontCharType, SmallVec };
use specs::{World, Entity};
use crate::rect::*;
use std::cmp::{max, min};

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 43;
const MAP_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(PartialEq, Clone, Copy)]
pub enum TileType{
    Wall,
    Floor,
}

pub struct Map{
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub currently_visible_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map{
    /// map x, y index into 1D vector idx
    pub fn xy_idx(&self, x: i32, y: i32) -> usize{
        (y as usize * self.width as usize) + x as usize
    }

    /// Fill room area with tiles
    fn apply_room_to_map(&mut self, room: &Rect){
        for y in room.y1..=room.y2{
            for x in room.x1..=room.x2{
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    /// Take in a map, starting and ending x value for tunnel, y value for tunnel elevation.
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32){
        for x in min(x1, x2)..=max(x1, x2){
            let idx = self.xy_idx(x, y);
    
            if idx < MAP_COUNT {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    /// Apply a vertical tunnel that begins at y1 and ends at y2 along a given x value on a map.
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32){
        for y in min(y1, y2)..=max(y1, y2){
            let idx = self.xy_idx(x, y);
    
            if idx < MAP_COUNT{
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn is_position_valid(&self, x: i32, y: i32) -> bool{
        if x < 1 || x > self.width-1 || y < 1 || y > self.height-1{
            return false;
        }

        !self.blocked_tiles[self.xy_idx(x, y)]
    }

    pub fn populate_blocked_tiles(&mut self){
        self.blocked_tiles = self.tiles
            .iter()
            .map(|tile| *tile == TileType::Wall)
            .collect();
    }

    pub fn clear_tile_content(&mut self){
        self.tile_content
            .iter_mut()
            .for_each(|t| t.clear());
    }

    pub fn map_with_rooms_and_corridors() -> Map{
        let mut map = Map{
            tiles: vec![TileType::Wall; MAP_COUNT],
            rooms: Vec::new(),
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
            revealed_tiles: vec![false; MAP_COUNT],
            currently_visible_tiles: vec![false; MAP_COUNT],
            blocked_tiles: vec![false; MAP_COUNT],
            tile_content: vec![Vec::new(); MAP_COUNT],
        };
    
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
    
        let mut rng = RandomNumberGenerator::new();
    
        for _ in 0..MAX_ROOMS{
            let room_width = rng.range(MIN_SIZE, MAX_SIZE);
            let room_height = rng.range(MIN_SIZE, MAX_SIZE);
    
            // final -1 because the borders are all walls
            let room_x = rng.range(1, map.width-room_width-1);
            let room_y = rng.range(1, map.height-room_height-1);
    
            let new_room = Rect::new(room_x, room_y, room_width, room_height);
    
            let any_intersecting_rooms = map.rooms.iter()
                .any(|r| new_room.intersect(&r));
    
            if !any_intersecting_rooms {
                map.apply_room_to_map(&new_room);
    
                if !map.rooms.is_empty(){
                    let (new_room_center_x, new_room_center_y) = new_room.center();
                    let (prev_room_center_x, prev_room_center_y) = map.rooms.last().unwrap().center();
    
                    // why does switching between new/prev for tunnel funcs do better results
                    // why does different order of tunnels do better
                    if rng.range(0, 2) == 0{
                        map.apply_horizontal_tunnel(new_room_center_x, prev_room_center_x, new_room_center_y);
                        map.apply_vertical_tunnel(new_room_center_y, prev_room_center_y, prev_room_center_x);
                    } else {
                        map.apply_vertical_tunnel(new_room_center_y, prev_room_center_y, new_room_center_x);
                        map.apply_horizontal_tunnel(new_room_center_x, prev_room_center_x, prev_room_center_y);
                    }
    
                }
    
                map.rooms.push(new_room);
            }
    
        }

        map
    }
}

impl BaseMap for Map{
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();

        let x = (idx % self.width as usize) as i32;
        let y = (idx / self.width as usize) as i32;

        let width_usize = self.width as usize;

        // cardinal directions
        if self.is_position_valid(x-1, y){ exits.push((idx-1, 1.0)); } 
        if self.is_position_valid(x+1, y){ exits.push((idx+1, 1.0)); }
        if self.is_position_valid(x, y-1){ exits.push((idx-width_usize, 1.0)); }
        if self.is_position_valid(x, y+1){ exits.push((idx+width_usize, 1.0)); }

        // diagonals
        if self.is_position_valid(x-1, y-1){ exits.push((idx-width_usize-1, 1.45)); }
        if self.is_position_valid(x-1, y+1){ exits.push((idx+width_usize-1, 1.45)); }
        if self.is_position_valid(x+1, y+1){ exits.push((idx+width_usize+1, 1.45)); }
        if self.is_position_valid(x+1, y-1){ exits.push((idx-width_usize+1, 1.45)); }


        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let width_usize = self.width as usize;
        let point1 = Point::new(idx1 % width_usize, idx1 / width_usize);
        let point2 = Point::new(idx2 % width_usize, idx2 / width_usize);
        rltk::DistanceAlg::Pythagoras.distance2d(point1, point2)
    }
}

impl Algorithm2D for Map{
    fn dimensions(&self) -> rltk::Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(world: &World, context: &mut Rltk) {
    let map = world.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for (idx, tile) in map.tiles.iter().enumerate(){

        // we only want to draw tiles that have been revealed either in the past or present (meaning currently visible)
        if map.revealed_tiles[idx]{
            let glyph: FontCharType;
            let mut fg: RGB;

            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                    
                },
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                },
            }

            // if something is revealed meaning we've seen it, but we can't currently see it. It's not in our current fov
            if !map.currently_visible_tiles[idx]{
                fg = fg.to_greyscale();
            }

            context.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }

        // update x, y for terminal
        x += 1;
        if x > MAP_WIDTH-1{
            x = 0;
            y += 1;
        }
    }

}


// First map created. Creates boundaries around border and randomly places 400 walls.
// pub fn first_map() -> Vec<TileType>{
//     // init fixed array with value
//     let mut map = vec![TileType::Floor; 80*50];

//     // make wall along top/bottom boundaries
//     for x in 0..80{
//         map[xy_idx(x, 0)] = TileType::Wall;
//         map[xy_idx(x, 49)] = TileType::Wall;
//     }

//     // make walls along left/right boundaries
//     for y in 0..50{
//         map[xy_idx(0, y)] = TileType::Wall;
//         map[xy_idx(79, y)] = TileType::Wall;
//     }

//     // make random walls
//     let mut rng = rltk::RandomNumberGenerator::new();

//     for _i in 0..400{
//         let x = rng.roll_dice(1, 79);
//         let y = rng.roll_dice(1, 49);

//         let random_idx = xy_idx(x, y);
//         let map_center_idx = xy_idx(40, 25);

//         if random_idx != map_center_idx {
//             map[random_idx] = TileType::Wall;
//         }
//     }

//     map
// }


