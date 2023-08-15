use rltk::{GameState, Rltk, RltkBuilder, Point};
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
mod monster_ai_system;
pub use crate::monster_ai_system::*;
mod map_indexing_system;
pub use crate::map_indexing_system::*;
mod melee_combat_system;
pub use crate::melee_combat_system::*;
mod damage_system;
pub use crate::damage_system::*;
mod gui;
pub use crate::gui::*;
mod gamelog;
pub use crate::gamelog::*;
mod spawner;
pub use crate::spawner::*;

#[derive(PartialEq, Clone, Copy)]
pub enum RunState{
    PreRun, // init
    AwaitingInput, // waiting for player to make action
    PlayerTurn, // update systems after player turn
    MonsterTurn, // run and update monsters
}

pub struct State {
    pub world: World,
}
impl State{
    pub fn run_systems(&mut self){
        // let mut up_walker_system = UpWalkerSystem{ };
        // up_walker_system.run_now(&self.world);

        // fov system
        let mut fov_system = FovSystem{ };
        fov_system.run_now(&self.world);

        // monster ai system
        let mut monster_ai_system = MonsterAI{ };
        monster_ai_system.run_now(&self.world);

        // map indexing system
        let mut map_indexing_system = MapIndexingSystem{ };
        map_indexing_system.run_now(&self.world);

        // melee combat system
        let mut melee_combat_system = MeleeCombatSystem{ };
        melee_combat_system.run_now(&self.world);

        // damage system
        let mut damage_system = DamageSystem{ };
        damage_system.run_now(&self.world);

        self.world.maintain();
    }
}
impl GameState for State {
    fn tick(&mut self, context : &mut Rltk) {
        context.cls();

        let mut run_state = *self.world.fetch::<RunState>();

        match run_state{
            RunState::PreRun => {
                self.run_systems();
                run_state = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                run_state = player_input(self, context);
            },
            RunState::PlayerTurn => {
                self.run_systems();
                run_state = RunState::MonsterTurn;
            },
            RunState::MonsterTurn => {
                self.run_systems();
                run_state = RunState::AwaitingInput;
            },
        }

        {
            let mut new_run_state = self.world.write_resource::<RunState>();
            *new_run_state = run_state;
        }

        // delete dead entities
        delete_dead_entities(&mut self.world);

        // draw map
        // let map = self.world.fetch::<Map>();
        draw_map(&self.world, context);

        // draw gui
        gui::draw_ui(&self.world, context);
        

        // render entities with renderable and position components
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        let map = self.world.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join(){
            let idx = map.xy_idx(pos.x, pos.y);
            if map.currently_visible_tiles[idx]{
                context.set(pos.x, pos.y, render.foreground, render.background, render.symbol);
            }
            
        }

    }
}

fn main() -> rltk::BError {
    let mut game_state = State{
        world: World::new(),
    };

    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    context.with_post_scanlines(true);


    // register components to game, attributes an entity can have
    game_state.world.register::<Position>();
    game_state.world.register::<Renderable>();
    // game_state.world.register::<UpMover>();
    game_state.world.register::<Player>();
    game_state.world.register::<FOV>();
    game_state.world.register::<Monster>();
    game_state.world.register::<Name>();
    game_state.world.register::<BlocksTile>();
    game_state.world.register::<CombatStats>();
    game_state.world.register::<WantsToMelee>();
    game_state.world.register::<SuffersDamage>();
    
    
    let map: Map = Map::map_with_rooms_and_corridors();
    
    // get valid x, y for player
    let (player_x, player_y) = map.rooms[0].center();
    
    // create entities, something in the world with components
    // this is player entity
    let player_entity = spawner::player(&mut game_state.world, player_x, player_y);

    game_state.world.insert(rltk::RandomNumberGenerator::new());
    
    for room in map.rooms.iter().skip(1){
        spawner::spawn_entities_in_room(&mut game_state.world, room);
    }
    
    // make map resource availale to world
    game_state.world.insert(map);
    game_state.world.insert(Point::new(player_x, player_y));
    // insert player as resource into world
    game_state.world.insert(player_entity);
    // insert run state as resource
    game_state.world.insert(RunState::PreRun);
    game_state.world.insert(GameLog{ entries: vec!["Welcome!".to_string()]});
    
    rltk::main_loop(context, game_state)


}