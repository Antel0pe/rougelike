use std::{fs::{File, read_to_string, self}, path::Path};

use specs::{prelude::*, saveload::{MarkedBuilder, SerializeComponents, DeserializeComponents}};
use super::*;
use std::convert::Infallible;

pub const SAVE_FILE_PATH: &str = "./saved_game.json";

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
        let file_writer = File::create(SAVE_FILE_PATH).unwrap();
        let mut serializer = serde_json::Serializer::new(file_writer);

        serialize_components!(world, serializer, data, 
            Position, Renderable, Player, FOV, Monster, Name, BlocksTile, CombatStats, WantsToMelee, SuffersDamage, Item, ProvidesHealing, InBackpack, WantsToPickUpItem, WantsToUseItem, WantsToDropItem, Consumable, Ranged, InflictsDamage, AreaOfEffect, CausesConfusion, IsConfused, GivesMovementSpeed, HasMovementSpeedModifier, SerializationHelper);
    }


    let _ = world.delete_entity(save_helper);
}

pub fn does_save_exist() -> bool{
    Path::new(SAVE_FILE_PATH).exists()
}

macro_rules! deserialize_components {
    ($world:expr, $deserializer:expr, $data:expr, $( $type:ty ), *) => {
        $(
            DeserializeComponents::<Infallible, _>::deserialize(
                &mut (&mut $world.write_storage::<$type>(), ),
                &mut $data.0, // entities
                &mut $data.1, // marker
                &mut $data.2, // allocator
                &mut $deserializer,
            )
            .unwrap();
        )*
    };
}

pub fn load_game(world: &mut World){
    // first delete everything from current game
    {
        let mut entities_to_delete: Vec<Entity> = Vec::new();
        entities_to_delete.extend(world.entities().join());

        entities_to_delete.iter().for_each(|e|{
            world.delete_entity(*e)
                .expect("Could not delete entity.");
        });

    }

    let file_data = read_to_string(SAVE_FILE_PATH).unwrap();
    let mut deserializer = serde_json::Deserializer::from_str(&file_data);

    {
        // write to entities, components with serialize me, marker allocator
        let mut data = (
            &mut world.entities(),
            &mut world.write_storage::<SimpleMarker<SerializeMe>>(),
            &mut world.write_resource::<SimpleMarkerAllocator<SerializeMe>>()
        );

        deserialize_components!(world, deserializer, data,
            Position, Renderable, Player, FOV, Monster, Name, BlocksTile, CombatStats, WantsToMelee, SuffersDamage, Item, ProvidesHealing, InBackpack, WantsToPickUpItem, WantsToUseItem, WantsToDropItem, Consumable, Ranged, InflictsDamage, AreaOfEffect, CausesConfusion, IsConfused, GivesMovementSpeed, HasMovementSpeedModifier, SerializationHelper);

    }

    let mut serialization_helper_to_delete: Option<Entity> = None;
    {
        let entities = world.entities();
        let serialization_helper = world.read_storage::<SerializationHelper>();
        let player = world.read_storage::<Player>();
        let player_position = world.read_storage::<Position>();

        for (entity, serialization_helper) in (&entities, &serialization_helper).join(){
            let mut map = world.write_resource::<Map>();
            *map = serialization_helper.map.clone();
            map.load_from_save();

            serialization_helper_to_delete = Some(entity);
        }

        for (entity, _player, position) in (&entities, &player, &player_position).join(){
            let mut player_position = world.write_resource::<rltk::Point>();
            *player_position = rltk::Point::new(position.x, position.y);

            let mut player_resource = world.write_resource::<Entity>();
            *player_resource = entity;
        }
    }

    world.delete_entity(serialization_helper_to_delete.unwrap())
        .expect("Could not delete serialization helper.");

}

pub fn delete_save(){
    if Path::new(SAVE_FILE_PATH).exists(){
        fs::remove_file(SAVE_FILE_PATH)
            .expect("Could not delete save file.");
    }
}