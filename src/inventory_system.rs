use specs::prelude::*;

use crate::{GameLog, WantsToPickUpItem, Position, InBackpack, Name, WantsToDrinkPotion, CombatStats, ProvidesHealing, WantsToDropItem, Consumable};

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
                        WriteStorage<'a, WantsToDrinkPotion>,
                        WriteStorage<'a, CombatStats>,
                        ReadExpect<'a, Entity>,
                        ReadStorage<'a, ProvidesHealing>,
                        ReadStorage<'a, Consumable>,
                     );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, entities, names, mut wants_to_drink_potion, mut combat_stats, player_entity, provides_healing, consumables) = data;

        for (entity, drink_potion, stats) in (&entities, &wants_to_drink_potion, &mut combat_stats).join(){
            let potion = provides_healing.get(drink_potion.potion);

            match potion{
                None => {},
                Some(potion) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);

                    if entity == *player_entity{
                        gamelog.entries.push(format!("You drink the {} for {} hp", names.get(drink_potion.potion).unwrap().name, potion.heal_amount));
                    }     
                }
            }

            match consumables.get(drink_potion.potion){
                None => {},
                Some(_) => {
                    entities.delete(drink_potion.potion)
                        .expect("Could not delete consumable entity.");
                }
            }
        }

        wants_to_drink_potion.clear();
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