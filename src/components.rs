use specs::prelude::*;
use specs_derive::*;
use rltk::RGB;

#[derive(Component)] // Creates Vector storage of Self objs
pub struct Position{
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable{
    pub symbol: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}

#[derive(Component)]
/// Component to track FOV of things
pub struct FOV{
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    // if the player moves then the fov needs to be updated
    pub needs_update: bool,
}

#[derive(Component, Debug)]
pub struct Monster{
    
}

#[derive(Component, Debug, Clone,)]
pub struct Name{
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile{

}

#[derive(Component, Debug)]
pub struct CombatStats{
    pub max_hp: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, Debug, Clone)]
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