use specs::prelude::*;

use crate::{GameLog, WantsToPickUpItem, Position, InBackpack, Name, WantsToUseItem, CombatStats, ProvidesHealing, WantsToDropItem, InflictsDamage, Map, SuffersDamage, Consumable, AreaOfEffect, CausesConfusion, IsConfused, GivesMovementSpeed, HasMovementSpeedModifier, Equippable, Equipped, WantsToUnequipItem};

pub struct ItemCollectionSystem{ }

impl<'a> System<'a> for ItemCollectionSystem{
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, WantsToPickUpItem>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, InBackpack>,
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut gamelog, mut wants_to_pickup_item, mut position, names, mut backpack) = data;

        for pick_up_item in wants_to_pickup_item.join(){
            position.remove(pick_up_item.item);
            backpack.insert(pick_up_item.item, InBackpack { owner: pick_up_item.collected_by })
                .expect("Unable to insert item into backpack...");

            if pick_up_item.collected_by == *player_entity{
                gamelog.entries.push(format!("You picked up {}.", names.get(pick_up_item.item).unwrap().name));
            }
        }

        wants_to_pickup_item.clear();
    }
}

pub struct ItemUseSystem{ }

impl<'a> System<'a> for ItemUseSystem{
    type SystemData = ( WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, WantsToUseItem>,
                        WriteStorage<'a, CombatStats>,
                        ReadExpect<'a, Entity>,
                        ReadStorage<'a, ProvidesHealing>,
                        ReadStorage<'a, InflictsDamage>,
                        ReadExpect<'a, Map>,
                        WriteStorage<'a, SuffersDamage>,
                        WriteStorage<'a, Consumable>,
                        ReadStorage<'a, AreaOfEffect>,
                        ReadStorage<'a, CausesConfusion>,
                        WriteStorage<'a, IsConfused>,
                        ReadStorage<'a, GivesMovementSpeed>,
                        WriteStorage<'a, HasMovementSpeedModifier>,
                        ReadStorage<'a, Equippable>,
                        WriteStorage<'a, Equipped>,
                        WriteStorage<'a, InBackpack>,
                     );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut gamelog,
            entities, names,
            mut wants_to_use_item,
            mut combat_stats,
            player_entity,
            provides_healing,
            inflicts_damage,
            map,
            mut suffers_damage,
            mut consumables,
            area_of_effect,
            causes_confusion,
            mut is_confused,
            gives_movement_speed,
            mut has_movement_speed_modifier,
            equippable,
            mut equipped,
            mut backpack,
        ) = data;

        for (entity, use_item) in (&entities, &wants_to_use_item).join(){
            let mut used_item = false;

            // find targets
            let mut targets: Vec<Entity> = Vec::new();
            match use_item.target{
                // self targeted effect
                None => {
                    targets.push(*player_entity);
                },
                Some(target) => {
                    match area_of_effect.get(use_item.item){
                        // single target effect
                        None => {   
                            let map_idx = map.xy_idx(target.x, target.y);
                            targets.extend(map.tile_content[map_idx].iter());
                        },
                        Some(aoe) =>{
                            let effect_radius = rltk::field_of_view(target, aoe.radius, &*map);
                            targets = effect_radius.iter() // for each point fov
                                .filter(|p| Map::is_idx_valid(p.x, p.y)) // filter those that are valid idx
                                .map(|p| map.xy_idx(p.x, p.y)) // map them to 1d idx
                                .fold(targets, |mut acc, idx|{ // add all mobs in that tile
                                    acc.extend(map.tile_content[idx].iter());
                                    acc
                                });
                        }
                    }
                }
            }

            if let Some(confusing_item) = causes_confusion.get(use_item.item){
                for target in targets.iter(){
                    is_confused.insert(*target, IsConfused { turns: confusing_item.turns })
                        .expect("Could not add is confused component to target.");

                    if entity == *player_entity{
                        gamelog.entries.push(format!("You use {} on {} confusing them for {} turns",
                            names.get(use_item.item).unwrap().name, names.get(*target).unwrap().name, confusing_item.turns));
                    }
                }

                used_item = true;
            }

            if let Some(movement_speed_item) = gives_movement_speed.get(use_item.item){
                for target in targets.iter(){
                    has_movement_speed_modifier.insert(*target, HasMovementSpeedModifier { speed_modifier: movement_speed_item.speed_modifier, max_turns: movement_speed_item.turns, turns_used: 0 })
                        .expect("Could not add movement speed modifier.");

                    if entity == *player_entity{
                        gamelog.entries.push(format!("You use {} to gain a movement speed of x{} for {} turns",
                            names.get(use_item.item).unwrap().name, movement_speed_item.speed_modifier, movement_speed_item.turns));
                    }
                }

                used_item = true;
            }

            if let Some(potion) = provides_healing.get(use_item.item){
                for target in targets.iter(){
                    if let Some(stats) = combat_stats.get_mut(*target){
                        stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);

                        if entity == *player_entity{
                            gamelog.entries.push(format!("You drink the {} for {} hp", names.get(use_item.item).unwrap().name, potion.heal_amount));
                        }  
                    }   
                }

                used_item = true;
            }

            if let Some(item) = inflicts_damage.get(use_item.item){
                for mob in targets.iter(){
                    SuffersDamage::new_damage(&mut suffers_damage, *mob, item.damage);

                    if entity == *player_entity{
                        gamelog.entries.push(format!("You hit {} for {} hp with {}.",
                            names.get(*mob).unwrap().name, item.damage, names.get(use_item.item).unwrap().name));
                    }
                }

                used_item = true;
            }

            if let Some(can_equip) = equippable.get(use_item.item){
                let target_slot = can_equip.slot;
                let target_entity = targets[0];

                // find items in existing slot
                let mut items_to_unequip = Vec::new();
                for (entity, equipped, name) in (&entities, &equipped, &names).join(){
                    if equipped.owner == target_entity && equipped.slot == target_slot{
                        items_to_unequip.push(entity);

                        if target_entity == *player_entity{
                            gamelog.entries.push(format!("You unequip {}", name.name));
                        }
                    }
                }

                // unequip items and place in backpack
                for item in items_to_unequip.iter(){
                    equipped.remove(*item);
                    backpack.insert(*item, InBackpack { owner: target_entity })
                        .expect("Could not insert unequipped item into backpack.");
                }

                // equip new item
                equipped.insert(use_item.item, Equipped { owner: target_entity, slot: target_slot })
                    .expect("Could not equip item.");
                backpack.remove(use_item.item);
                if target_entity == *player_entity{
                    gamelog.entries.push(format!("You equip {}", names.get(use_item.item).unwrap().name))
                }

            }

            if let Some(consumable_item) = consumables.get_mut(use_item.item){
                if used_item{
                    consumable_item.charges -= 1;
    
                    if consumable_item.charges <= 0{
                        entities.delete(use_item.item)
                            .expect("Could not delete consumable entity.");
                    }
                }
            }
        }

        wants_to_use_item.clear();
    }
}

pub struct ItemDropSystem{ }

impl<'a> System<'a> for ItemDropSystem{
    type SystemData = ( ReadExpect<'a, Entity>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDropItem>,
                        WriteStorage<'a, InBackpack>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Name>,
                        WriteExpect<'a, GameLog>,
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, entities, mut wants_to_drop_item, mut in_backpack, mut positions, names, mut gamelog) = data;


        for (item, entity) in (&wants_to_drop_item, &entities).join(){
            let dropped_position: &Position = positions.get(entity).unwrap();

            positions.insert(item.item, Position { x: dropped_position.x, y: dropped_position.y })
                .expect("Unable to drop item.");
            
            in_backpack.remove(item.item);

            if entity == *player_entity{
                gamelog.entries.push(format!("You drop a {}", names.get(item.item).unwrap().name));
            }

        }

        wants_to_drop_item.clear();
    }
}

pub struct ItemUnequipSystem{ }

impl<'a> System<'a> for ItemUnequipSystem{
    type SystemData = ( WriteStorage<'a, WantsToUnequipItem>,
                        WriteStorage<'a, InBackpack>,
                        WriteStorage<'a, Equipped>,
                        Entities<'a>,
                        WriteExpect<'a, GameLog>,
                        ReadExpect<'a, Entity>,
                        ReadStorage<'a, Name>,
                    );

    fn run(&mut self, data: Self::SystemData) {
        let ( 
                mut wants_to_unequip_item,
                mut in_backpack, 
                mut equipped,
                entities,
                mut gamelog,
                player_entity,
                names,
            )
            = data;


        for (wants_to_unequip, entity) in (&wants_to_unequip_item, &entities).join(){
            equipped.remove(wants_to_unequip.item);
            in_backpack.insert(wants_to_unequip.item, InBackpack { owner: entity })
                .expect("Could not unequip item");

            if entity == *player_entity{
                gamelog.entries.push(format!("You unequip the {}", names.get(wants_to_unequip.item).unwrap().name));
            }

        }

        wants_to_unequip_item.clear();
    }
}