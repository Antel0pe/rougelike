use specs::prelude::*;

use crate::{WantsToMelee, CombatStats, SuffersDamage, Name, GameLog, MeleePowerBonus, DefenseBonus, Equipped};

pub struct MeleeCombatSystem{ }

impl<'a> System<'a> for MeleeCombatSystem{
    type SystemData = ( WriteStorage<'a, WantsToMelee>,
                        ReadStorage<'a, CombatStats>,
                        WriteStorage<'a, SuffersDamage>,
                        ReadStorage<'a, Name>,
                        Entities<'a>,
                        WriteExpect<'a, GameLog>,
                        ReadStorage<'a, MeleePowerBonus>,
                        ReadStorage<'a, DefenseBonus>,
                        ReadStorage<'a, Equipped>,
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut wants_to_melee, combat_stats, mut suffers_damage, names, entities, mut gamelog, melee_bonus, defense_bonus, equipped,) = data;

        for (stats, name, entity, wants_to_melee) in (&combat_stats, &names, &entities, &mut wants_to_melee).join(){

            if stats.hp < 0 { continue; }

            let target_name = names.get(wants_to_melee.target)
                .map(|name| name.name.clone())
                .unwrap_or_else(|| "Unnamed entity".to_string());

            // calculate attacker's attack bonus
            let mut offensive_bonus = 0;
            for (power_bonus, equipped, _entity) in (&melee_bonus, &equipped, &entities).join(){
                if equipped.owner == entity{
                    offensive_bonus += power_bonus.power;
                }
            }

            if let Some(target_combat_stats) = combat_stats.get(wants_to_melee.target){
                if target_combat_stats.hp > 0{

                    // calculate target's defense bonus
                    let mut defensive_bonus = 0;
                    for (_item, defense_bonus, equipped) in (&entities, &defense_bonus, &equipped).join(){
                        if equipped.owner == wants_to_melee.target{
                            defensive_bonus += defense_bonus.defense;
                        }
                    }

                    let damage = i32::max(0, (stats.attack + offensive_bonus) - (target_combat_stats.defense + defensive_bonus));

                    if damage == 0{
                        gamelog.entries.push(format!("{} could not damage {}...", name.name, target_name));
                    } else {
                        gamelog.entries.push(format!("{} hits {} for {} hp!", name.name, target_name, damage));
                        SuffersDamage::new_damage(&mut suffers_damage, wants_to_melee.target, damage);
                    }
                    
                }
                

            }
        }

        // after they've attacked, they don't attack until they have the component re-added
        wants_to_melee.clear();
    }
}