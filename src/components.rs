use specs::prelude::*;
use specs_derive::*;
use rltk::RGB;
use serde::{Serialize, Deserialize};
use specs::saveload::*;
use specs::error::*;

use crate::Map;

#[derive(Component, ConvertSaveload, Clone)] // Creates Vector storage of Self objs
pub struct Position{
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable{
    pub symbol: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB,
    pub render_order: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Player{ }

#[derive(Component, ConvertSaveload, Clone)]
/// Component to track FOV of things
pub struct FOV{
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    // if the player moves then the fov needs to be updated
    pub needs_update: bool,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster{
    
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct Name{
    pub name: String,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile{

}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CombatStats{
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct SuffersDamage{
    pub amount: Vec<i32>,
}

impl SuffersDamage{
    pub fn new_damage(suffers_damage: &mut WriteStorage<SuffersDamage>, entity: Entity, amount: i32){
        // if the entity already has a suffers damage component, simply add the dmg suffered
        if let Some(damaged_entity) = suffers_damage.get_mut(entity){
            damaged_entity.amount.push(amount);
        } else { // if the entity does not have a suffers damage component, initialize it and add the dmg suffered
            let suffered_damage = SuffersDamage{ amount: vec![amount] };
            suffers_damage.insert(entity, suffered_damage)
                .expect("Unable to isert damage.");
        }
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item{ }

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct ProvidesHealing{
    pub heal_amount: i32,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InBackpack{
    pub owner: Entity,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToPickUpItem{
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct WantsToUseItem{
    pub item: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToDropItem{
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Consumable{
    pub charges: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Ranged{
    pub range: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct InflictsDamage{
    pub damage: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect{
    pub radius: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CausesConfusion{
    pub turns: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct IsConfused{
    pub turns: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct GivesMovementSpeed{
    pub speed_modifier: i32,
    pub turns: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct HasMovementSpeedModifier{
    pub speed_modifier: i32,
    pub max_turns: i32,
    pub turns_used: i32,
}

// serialization helpers

pub struct SerializeMe;

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct SerializationHelper{
    pub map: Map,
}