use std::collections::HashMap;

use crate::{ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, CausesConfusion, GivesMovementSpeed, SerializeMe, RandomTable, Equippable, EquipmentSlot, MeleePowerBonus, DefenseBonus};

use super::{Player, Position, Renderable, FOV, Name, CombatStats, Monster, BlocksTile, Rect, Item};
use specs::{prelude::*, saveload::{MarkedBuilder, SimpleMarker}};
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
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
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

// TODO: Refactor and extract duplicated logic
// monster and item spawning is the exact same
pub fn spawn_entities_in_room(world: &mut World, room: &Rect, map_depth: i32){
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<(i32, i32), String> = HashMap::new();

    {    
        let mut rng = world.write_resource::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS_PER_ROOM + 3) + (map_depth - 1) - 3; // possibility of less than 0 monsters

        for _ in 0..num_spawns{
            let mut added = false;
            let mut tries = 0;

            while !added && tries < 20{
                let x = room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1));
                let y = room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1));

                if !spawn_points.contains_key(&(x, y)){
                    spawn_points.insert((x, y), spawn_table.roll(&mut rng));
                    added = true;
                } else {
                    tries += 1;
                }

            }
        }
    }

    for ((x, y), name) in spawn_points.iter(){
        match name.as_ref(){
            "Goblin" => goblin(world, *x, *y),
            "Orc" => orc(world, *x, *y),
            "Health Potion" => health_potion(world, *x, *y),
            "Fireball Scroll" => fireball_spell(world, *x, *y),
            "Confusion Scroll" => confusion_spell(world, *x, *y),
            "Magic Missile Scroll" => magic_missile_scroll(world, *x, *y),
            "Dagger" => dagger(world, *x, *y),
            "Shield" => shield(world, *x, *y),
            "Tower Shield" => tower_shield(world, *x, *y),
            "Longsword" => longsword(world, *x, *y),
            _ => {},
        }
    }

    
}

pub fn orc(world: &mut World, x: i32, y: i32){
    monster(world, x, y, rltk::to_cp437('g'), "Goblin");
}

pub fn goblin(world: &mut World, x: i32, y: i32){
    monster(world, x, y, rltk::to_cp437('o'), "Orc");
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
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn room_table(map_depth: i32) -> RandomTable{
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
        .add("Dagger", 3)
        .add("Shield", 3)
        .add("Tower Shield", map_depth-1)
        .add("Longsword", map_depth-1)
}

pub fn dagger(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437('/'),
            foreground: RGB::named(rltk::CYAN),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Dagger".to_string() })
        .with(Item{ })
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn shield(world: &mut World, x:i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437('('),
            foreground: RGB::named(rltk::CYAN),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Shield".to_string() })
        .with(Item{ })
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenseBonus{ defense: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn longsword(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437('/'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Longsword".to_string() })
        .with(Item{ })
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn tower_shield(world: &mut World, x: i32, y: i32){
    world.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            symbol: rltk::to_cp437('('),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Tower Shield".to_string() })
        .with(Item{ })
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenseBonus{ defense: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

