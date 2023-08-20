use crate::{ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, CausesConfusion, GivesMovementSpeed};

use super::{Player, Position, Renderable, FOV, Name, CombatStats, Monster, BlocksTile, Rect, Item};
use specs::prelude::*;
use rltk::{RGB, RandomNumberGenerator};

pub const MAX_MONSTERS_PER_ROOM: i32 = 4;
pub const MAX_ITEMS_PER_ROOM: i32 = 2;

pub fn player(world: &mut World, player_x: i32, player_y: i32) -> Entity {
    world.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            symbol: rltk::to_cp437('o'),
            foreground: RGB::named(rltk::PURPLE),
            background: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Player{ })
        .with(FOV{ visible_tiles: Vec::new(), range: 8, needs_update: true, })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, attack: 5, defense: 2, })
        .build()
}

pub fn health_potion(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437('¡'),
            foreground: RGB::named(rltk::MAGENTA),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Health Potion".to_string() })
        .with(Item{ })
        .with(ProvidesHealing{ heal_amount: 8 })
        .with(Consumable{ charges: 3 })
        .build();
}

pub fn magic_missile_scroll(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437(')'),
            foreground: RGB::named(rltk::CYAN),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Magic Missile Scroll".to_string() })
        .with(Item{ })
        .with(Consumable{ charges: 1 })
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .build();
}

pub fn fireball_spell(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437(')'),
            foreground: RGB::named(rltk::ORANGE),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Fireball spell".to_string() })
        .with(Item{ })
        .with(Consumable{ charges: 1 })
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 20 })
        .with(AreaOfEffect{ radius: 3 })
        .build();
}

pub fn confusion_spell(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437(')'),
            foreground: RGB::named(rltk::PINK),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Confusion spell".to_string() })
        .with(Item{ })
        .with(Consumable{ charges: 1 })
        .with(Ranged{ range: 6 })
        .with(CausesConfusion{ turns: 3 })
        .build();
}

pub fn dash_boots(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437('b'),
            foreground: RGB::named(rltk::BROWN1),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Dash boots".to_string() })
        .with(Item{ })
        .with(Consumable{ charges: 1 })
        .with(GivesMovementSpeed{ speed_modifier: 2, turns: 3 })
        .build();
}

// TODO: Refactor and extract duplicated logic
// monster and item spawning is the exact same
pub fn spawn_entities_in_room(world: &mut World, room: &Rect){
    let mut monster_spawn_points: Vec<(i32, i32)> = Vec::new();
    let mut item_spawn_points: Vec<(i32, i32)> = Vec::new();

    {    
        let mut rng = world.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.roll_dice(1, MAX_MONSTERS_PER_ROOM + 2) - 3; // possibility of less than 0 monsters
        let num_items = rng.roll_dice(1, MAX_ITEMS_PER_ROOM + 2) - 3;

        for _ in 0..num_monsters{
            let mut valid_spawn_point_found = false;
            while !valid_spawn_point_found{
                let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
                let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));

                if !monster_spawn_points.contains(&(x, y)){
                    monster_spawn_points.push((x, y));
                    valid_spawn_point_found = true;
                }
            }
        }

        for _ in 0..num_items{
            let mut valid_spawn_point_found = false;
            while !valid_spawn_point_found{
                let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
                let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));

                if !item_spawn_points.contains(&(x, y)){
                    item_spawn_points.push((x, y));
                    valid_spawn_point_found = true;
                }
            }
        }
    }

    for (x, y) in monster_spawn_points.iter(){
        random_monster(world, *x, *y);
    }

    for (x, y) in item_spawn_points.iter(){
        random_item(world, *x, *y);
    }

}

pub fn random_item(world: &mut World, x: i32, y: i32){
    let roll: i32;

    {
        let mut rng = world.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 5);
    }

    match roll{
        1 => health_potion(world, x, y),
        2 => fireball_spell(world, x, y),
        3 => confusion_spell(world, x, y),
        4 => dash_boots(world, x, y),
        _ => magic_missile_scroll(world, x, y),
    }
}

pub fn random_monster(world: &mut World, x: i32, y: i32) -> Entity{
    let roll: i32;
    {
        let mut rng = world.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 2);
    }

    match roll {
        1 => orc(world, x, y),
        _ => goblin(world, x, y),
    }
}

pub fn orc(world: &mut World, x: i32, y: i32) -> Entity{
    monster(world, x, y, rltk::to_cp437('g'), "Goblin")
}

pub fn goblin(world: &mut World, x: i32, y: i32) -> Entity{
    monster(world, x, y, rltk::to_cp437('o'), "Orc")
}

pub fn monster<S: ToString> (world: &mut World, x: i32, y: i32, glyph: rltk::FontCharType, name: S) -> Entity{
    world.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            symbol: glyph,
            foreground: RGB::named(rltk::RED),
            background: RGB::named(rltk::BLACK),
            render_order: 1,
        })
        .with(FOV{ visible_tiles: Vec::new(), range: 8, needs_update: true, })
        .with(Monster{ })
        .with(Name{ name: name.to_string() })
        .with(BlocksTile{ })
        .with(CombatStats{ max_hp: 16, hp: 16, attack: 4, defense: 1, })
        .build()
}