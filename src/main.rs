use rltk::{GameState, Rltk, RltkBuilder, Point};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

extern crate serde;

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
mod inventory_system;
pub use crate::inventory_system::*;
mod movement_speed_modifier;
pub use crate::movement_speed_modifier::*;
mod save_system;
pub use crate::save_system::*;
mod random_table;
pub use crate::random_table::*;


#[derive(PartialEq, Clone, Copy)]
pub enum RunState{
    PreRun, // init
    AwaitingInput, // waiting for player to make action
    PlayerTurn, // update systems after player turn
    MonsterTurn, // run and update monsters
    InInventory,
    ShowDropItem,
    ShowTargetting{ range: i32, item: Entity, },
    MainMenu{ menu_selection: gui::MainMenuSelection },
    SaveGame,
    DescendFloor,
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

        // pick up item system
        let mut item_collection_system = ItemCollectionSystem{ };
        item_collection_system.run_now(&self.world);

        let mut item_use_system = ItemUseSystem{ };
        item_use_system.run_now(&self.world);

        let mut item_drop_system = ItemDropSystem{ };
        item_drop_system.run_now(&self.world);

        let mut movement_speed_modifier = MovementSpeedModifier{ };
        movement_speed_modifier.run_now(&self.world);

        self.world.maintain();
    }

    pub fn entities_to_remove_on_depth_change(&mut self) -> Vec<Entity>{
        let entities = self.world.entities();
        let players = self.world.read_storage::<Player>();
        let backpacks = self.world.read_storage::<InBackpack>();
        let player_entity = self.world.fetch::<Entity>();
        let equipped = self.world.read_storage::<Equipped>();
    
        let mut entities_to_delete = Vec::new();
        for entity in entities.join(){
            let mut should_delete = true;
    
            if let Some(_p) = players.get(entity){
                should_delete = false;
            }
    
            if let Some(backpack) = backpacks.get(entity){
                if backpack.owner == *player_entity{
                    should_delete = false;
                }
            }

            if let Some(equip) = equipped.get(entity){
                if equip.owner == *player_entity{
                    should_delete = false;
                }
            }
    
            if should_delete{
                entities_to_delete.push(entity);
            }
        }
    
        entities_to_delete
    }

    pub fn goto_next_level(&mut self){
        let entities_to_delete = self.entities_to_remove_on_depth_change();

        for entity in entities_to_delete{
            self.world.delete_entity(entity)
                .expect("Could not delete entity on level change.");
        }

        let new_level_map: Map;
        let current_depth: i32;
        {
            let mut map = self.world.write_resource::<Map>();
            current_depth = map.depth;
            *map = Map::map_with_rooms_and_corridors(current_depth+1);
            new_level_map = map.clone();
        }

        for room in new_level_map.rooms.iter().skip(1){
            spawn_entities_in_room(&mut self.world, room, current_depth+1);
        }

        let (player_x, player_y) = new_level_map.rooms[0].center();
        let mut player_position = self.world.write_resource::<Point>();
        *player_position = Point::new(player_x, player_y);

        let mut position_components = self.world.write_storage::<Position>();
        let player_entity = self.world.fetch::<Entity>();

        if let Some(player_position_component) = position_components.get_mut(*player_entity){
            player_position_component.x = player_x;
            player_position_component.y = player_y;

        }

        let mut fov = self.world.write_storage::<FOV>();
        if let Some(player_fov) = fov.get_mut(*player_entity){
            player_fov.needs_update = true;
        }

        let mut gamelog = self.world.fetch_mut::<GameLog>();
        gamelog.entries.push("You descend a level and get some health.".to_string());

        let mut combat_stats = self.world.write_storage::<CombatStats>();
        if let Some(player_stats) = combat_stats.get_mut(*player_entity){
            player_stats.hp = i32::max(player_stats.hp, player_stats.max_hp/2);
        }

    }
    
}
impl GameState for State {
    fn tick(&mut self, context : &mut Rltk) {
        context.cls();
    
        let mut run_state = *self.world.fetch::<RunState>();

        // don't do rendering if we are in main menu
        match run_state {
            RunState::MainMenu { .. } => {},
            _ => {
                draw_map(&self.world, context);
                gui::draw_ui(&self.world, context);

                // render entities with renderable and position components
                let positions = self.world.read_storage::<Position>();
                let renderables = self.world.read_storage::<Renderable>();
                let map = self.world.fetch::<Map>();

                let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

                for (pos, render) in data.iter(){
                    let idx = map.xy_idx(pos.x, pos.y);
                    if map.currently_visible_tiles[idx]{
                        context.set(pos.x, pos.y, render.foreground, render.background, render.symbol);
                    }
                    
                }
            }
        }

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
            RunState::InInventory => {
                let (item_menu_result, selected_entity) = gui::show_inventory(&mut self.world, context);

                match item_menu_result{
                    ItemMenuResult::Exit => run_state = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {},
                    ItemMenuResult::Selected => {
                        let selected_item = selected_entity.unwrap();
                        let ranged_items = self.world.read_storage::<Ranged>();

                        if let Some(ranged_item) = ranged_items.get(selected_item){
                            run_state = RunState::ShowTargetting { range: ranged_item.range, item: selected_item }
                        } else {
                            let mut wants_to_use_item = self.world.write_storage::<WantsToUseItem>();
    
                            wants_to_use_item.insert(*self.world.fetch::<Entity>(), WantsToUseItem { item: selected_item, target: None })
                                .expect("Unable to insert intent to use item for player.");
    
                            run_state = RunState::PlayerTurn;
                        }

                    }
                }
            },
            RunState::ShowDropItem => {
                let (menu_result, entity) = gui::show_drop_item_menu(&mut self.world, context);

                match menu_result{
                    ItemMenuResult::Exit => run_state = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {},
                    ItemMenuResult::Selected => {
                        let dropped_item = entity.unwrap();
                        let mut wants_to_drop_item = self.world.write_storage::<WantsToDropItem>();

                        wants_to_drop_item.insert(*self.world.fetch::<Entity>(), WantsToDropItem { item: dropped_item })
                            .expect("Could not add WantsToDropItem component to item for player.");

                        run_state = RunState::PlayerTurn;
                    },
                }
            },
            RunState::ShowTargetting { range, item } => {
                let (item_menu_result, selected_point) = gui::show_ranged_targeting(&mut self.world, context, range);

                match item_menu_result{
                    ItemMenuResult::Exit => run_state = RunState::AwaitingInput,
                    ItemMenuResult::NoResponse => {},
                    ItemMenuResult::Selected => {
                        let mut wants_to_use_item = self.world.write_storage::<WantsToUseItem>();
                        wants_to_use_item.insert(*self.world.fetch::<Entity>(), WantsToUseItem { item: item, target: selected_point })
                            .expect("Could not use targeted item");
                        run_state = RunState::PlayerTurn;

                    }
                }
            },
            RunState::MainMenu { .. } => {
                let selection = gui::main_menu(&mut self.world, context);

                match selection {
                    MainMenuResult::NoSelection { selected } =>{
                        run_state = RunState::MainMenu { menu_selection: selected };
                    },
                    MainMenuResult::Selected { selected } =>{
                        match selected{
                            MainMenuSelection::NewGame =>{
                                run_state = RunState::PreRun;
                            },
                            MainMenuSelection::LoadGame =>{
                                load_game(&mut self.world);
                                run_state = RunState::AwaitingInput;
                                delete_save();
                            },
                            MainMenuSelection::Quit =>{
                                ::std::process::exit(0);
                            }
                        }
                    },
                }
            },
            RunState::SaveGame =>{
                save_game(&mut self.world);                
                ::std::process::exit(0);
                // run_state = RunState::MainMenu { menu_selection: MainMenuSelection::LoadGame };
            },
            RunState::DescendFloor=>{
                self.goto_next_level();
                run_state = RunState::PreRun;
            }
        }

        {
            let mut new_run_state = self.world.write_resource::<RunState>();
            *new_run_state = run_state;
        }

        // delete dead entities
        delete_dead_entities(&mut self.world);

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
    game_state.world.register::<Item>();
    game_state.world.register::<ProvidesHealing>();
    game_state.world.register::<InBackpack>();
    game_state.world.register::<WantsToPickUpItem>();
    game_state.world.register::<WantsToUseItem>();
    game_state.world.register::<WantsToDropItem>();
    game_state.world.register::<Consumable>();
    game_state.world.register::<Ranged>();
    game_state.world.register::<InflictsDamage>();
    game_state.world.register::<AreaOfEffect>();
    game_state.world.register::<CausesConfusion>();
    game_state.world.register::<IsConfused>();
    game_state.world.register::<GivesMovementSpeed>();
    game_state.world.register::<HasMovementSpeedModifier>();
    game_state.world.register::<SimpleMarker<SerializeMe>>();
    game_state.world.register::<SerializationHelper>();
    game_state.world.register::<Equippable>();
    game_state.world.register::<Equipped>();
    game_state.world.register::<MeleePowerBonus>();
    game_state.world.register::<DefenseBonus>();

    game_state.world.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    
    
    let map: Map = Map::map_with_rooms_and_corridors(1);
    
    // get valid x, y for player
    let (player_x, player_y) = map.rooms[0].center();
    
    // create entities, something in the world with components
    // this is player entity
    let player_entity = spawner::player(&mut game_state.world, player_x, player_y);

    game_state.world.insert(rltk::RandomNumberGenerator::new());
    
    for room in map.rooms.iter().skip(1){
        spawner::spawn_entities_in_room(&mut game_state.world, room, 1);
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