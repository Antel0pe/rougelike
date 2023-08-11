use rltk::{RandomNumberGenerator, FontCharType};
use rltk::{GameState, Rltk, RGB, RltkBuilder, Point};
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

#[derive(PartialEq, Clone, Copy)]
pub enum RunState{
    Paused,
    Running,
}

pub struct State {
    pub world: World,
    pub run_state: RunState,
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

        // delete dead entities
        delete_dead_entities(&mut self.world);


        if self.run_state == RunState::Running{
            // run system only if the game is in a running state
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            //handle player movement
            self.run_state = player_input(self, context);
        }

        // draw map
        // let map = self.world.fetch::<Map>();
        draw_map(&self.world, context);
        

        // render entities with renderable and position components
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        let map = self.world.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join(){
            let idx = map.xy_idx(pos.x, pos.y);
            if !map.currently_visible_tiles[idx]{
                continue;
            }

            context.set(pos.x, pos.y, render.foreground, render.background, render.symbol);
        }

    }
}

fn main() -> rltk::BError {
    let mut game_state = State{
        world: World::new(),
        run_state: RunState::Running,
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
    game_state.world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            symbol: rltk::to_cp437('o'),
            foreground: RGB::named(rltk::PURPLE),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player{ })
        .with(FOV{ visible_tiles: Vec::new(), range: 8, needs_update: true, })
        .with(Name{ name: "Player".to_string() })
        .with(BlocksTile{ })
        .with(CombatStats{ max_hp: 30, hp: 30, attack: 5, defense: 2, })
        .build();

    //monster spawner
    let mut rng = RandomNumberGenerator::new();

    for (i, room) in map.rooms.iter().skip(1).enumerate(){
        let (x, y) = room.center();

        let glyph: FontCharType;
        let monster_type: String;

        match rng.roll_dice(1, 2){
            1 => {
                monster_type = "Goblin".to_string();
                glyph = rltk::to_cp437('g');
                
            },
            _ => {
                monster_type = "Orc".to_string();
                glyph = rltk::to_cp437('o');
            }
        };

        game_state.world.create_entity()
        .with(Position { x: x, y: y })
        .with(Renderable {
            symbol: glyph,
            foreground: RGB::named(rltk::RED),
            background: RGB::named(rltk::BLACK),
        })
        .with(FOV{ visible_tiles: Vec::new(), range: 8, needs_update: true, })
        .with(Monster{ })
        .with(Name{ name: format!("{} #{}", &monster_type, i) })
        .with(BlocksTile{ })
        .with(CombatStats{ max_hp: 16, hp: 16, attack: 4, defense: 1, })
        .build();
    }

    // make map resource availale to world
    game_state.world.insert(map);
    game_state.world.insert(Point::new(player_x, player_y));


    rltk::main_loop(context, game_state)


}