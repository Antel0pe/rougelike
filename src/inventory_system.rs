use specs::prelude::*;

use crate::{GameLog, WantsToPickUpItem, Position, InBackpack, Name};

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