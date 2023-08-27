use specs::prelude::*;
use crate::{SuffersDamage, CombatStats, Player, GameLog, Name, RunState};

pub struct DamageSystem{ }

impl<'a> System<'a> for DamageSystem{
    type SystemData = ( WriteStorage<'a, SuffersDamage>,
                        WriteStorage<'a, CombatStats>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut suffers_damage, mut combat_stats) = data;

        for (suffers_damage, combat_stats) in (&suffers_damage, &mut combat_stats).join(){
            combat_stats.hp -= suffers_damage.amount.iter().sum::<i32>();
        }

        // clear suffers_damage from entites, it has been calculated and applied already
        suffers_damage.clear();
    }
}

pub fn delete_dead_entities(world: &mut World){
    let mut dead_entities: Vec<Entity> = Vec::new();

    // scope to appease borrow checker
    // world is borrowed immutably when getting combat stats
    // if this scope wasn't here, world would be borrowed again to drop combat stats
    // however when it tried to drop it at the end of the function, since world was borrowed mutably when deleting entities
    // it would be borrowed both immutably and mutably at the same time
    // not 100% confident with this
    {
        let combat_stats = world.read_storage::<CombatStats>();
        let entities = world.entities();
        let players = world.read_storage::<Player>();
        let mut gamelog = world.write_resource::<GameLog>();
        let names = world.read_storage::<Name>();

        for (combat_stats, entity) in (&combat_stats, &entities).join(){
            if combat_stats.hp <= 0 {
                match players.get(entity){
                    Some(_) => {
                        let mut run_state = world.write_resource::<RunState>();
                        *run_state = RunState::GameOver;
                    },
                    None => {
                        let entity_name = names.get(entity);
                        if let Some(name) = entity_name{
                            gamelog.entries.push(format!("{} is dead.", name.name));
                        }

                        dead_entities.push(entity);
                    }
                }

            }
        }
    }

    for dead in dead_entities{
        world.delete_entity(dead)
            .expect("Unable to delete entity.");
    }
}