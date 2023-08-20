use super::*;

pub struct MovementSpeedModifier{ }

impl<'a> System<'a> for MovementSpeedModifier{
    type SystemData = ( WriteStorage<'a, HasMovementSpeedModifier>,
                        Entities<'a>,
                        ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut has_movement_speed_modifier,
            entities,
            player_entity,
            mut gamelog,
        ) = data;

        let mut expired_move_speed: Vec<Entity> = Vec::new();
        for (entity, move_speed_modifier) in (&entities, &mut has_movement_speed_modifier).join(){
            if move_speed_modifier.turns_used >= move_speed_modifier.max_turns{
                expired_move_speed.push(entity);

                if entity == *player_entity{
                    gamelog.entries.push("Your move speed has expired...".to_string());
                }
            }
        }

        for entity in expired_move_speed.iter(){
            has_movement_speed_modifier.remove(*entity);
        }
    }
}