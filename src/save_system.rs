use std::fs::File;

use specs::{prelude::*, saveload::{MarkedBuilder, SerializeComponents}};
use super::*;
use std::convert::Infallible;

macro_rules! serialize_components {
    ($world: expr, $serializer: expr, $data: expr, $( $type:ty), *) => {
        $(
            SerializeComponents::<Infallible, SimpleMarker<SerializeMe>>::serialize(
                &( $world.read_storage::<$type>(), ),
                &$data.0,
                &$data.1,
                &mut $serializer,
            )
            .unwrap();
        )*
    };
}

pub fn save_game(world: &mut World){
    let map_copy = world.get_mut::<Map>().unwrap().clone();

    let save_helper = world.create_entity()
        .with(SerializationHelper{ map: map_copy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let data = (world.entities(), world.read_storage::<SimpleMarker<SerializeMe>>());
        let file_writer = File::create("./saved_game.json").unwrap();
        let mut serializer = serde_json::Serializer::new(file_writer);

        serialize_components!(world, serializer, data, 
            Position, Renderable, Player, FOV, Monster, Name, BlocksTile, CombatStats, WantsToMelee, SuffersDamage, Item, ProvidesHealing, InBackpack, WantsToPickUpItem, WantsToUseItem, WantsToDropItem, Consumable, Ranged, InflictsDamage, AreaOfEffect, CausesConfusion, IsConfused, GivesMovementSpeed, HasMovementSpeedModifier, SerializationHelper);
    }


    let _ = world.delete_entity(save_helper);
}