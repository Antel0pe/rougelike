use rltk::{GameState, Rltk, RGB, RltkBuilder};
use specs::prelude::*;


mod map;
pub use crate::map::*;
mod player;
pub use crate::player::*;
mod components;
pub use crate::components::*;
mod rect;
pub use crate::rect::*;
mod fov;
pub use crate::fov::*;


pub struct State {
    pub world: World,
}
impl State{
    pub fn run_systems(&mut self){
        // let mut up_walker_system = UpWalkerSystem{ };
        // up_walker_system.run_now(&self.world);

        let mut fov_system = FovSystem{ };
        fov_system.run_now(&self.world);
        self.world.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, context : &mut Rltk) {
        context.cls();

        self.run_systems();

        //handle player movement
        player_input(self, context);

        // draw map
        // let map = self.world.fetch::<Map>();
        draw_map(&self.world, context);
        

        // render entities with renderable and position components
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join(){
            context.set(pos.x, pos.y, render.foreground, render.background, render.symbol);
        }
    }
}

fn main() -> rltk::BError {
    let mut game_state = State{
        world: World::new(),
    };

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;


    // register components to game, attributes an entity can have
    game_state.world.register::<Position>();
    game_state.world.register::<Renderable>();
    // game_state.world.register::<UpMover>();
    game_state.world.register::<Player>();
    game_state.world.register::<FOV>();


    let map: Map = Map::map_with_rooms_and_corridors();

    // get valid x, y for player
    let (player_x, player_y) = map.rooms[0].center();

    // make map resource availale to world
    game_state.world.insert(map);

    // create entities, something in the world with components
    // this is player entity
    game_state.world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            symbol: rltk::to_cp437('o'),
            foreground: RGB::named(rltk::PURPLE),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player{ })
        .with(FOV{ visible_tiles: Vec::new(), range: 8, needs_update: true, })
        .build();


    rltk::main_loop(context, game_state)


}