use specs::{World, WorldExt, Join};
use rltk::{Rltk, RGB};

use crate::{CombatStats, Player, GameLog};

pub fn draw_ui(world: &World, context: &mut Rltk){
    // draw box around bottom bit of screen
    context.draw_box(0, 43, 79, 6, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    // draw hp bar
    let combat_stats = world.read_storage::<CombatStats>();
    let player = world.read_storage::<Player>();

    for (combat_stats, _player) in (&combat_stats, &player).join(){
        let player_hp = format!("HP: {} / {}", combat_stats.hp, combat_stats.max_hp);

        context.print_color(12, 43, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &player_hp);
        context.draw_bar_horizontal(28, 43, 51, combat_stats.hp, combat_stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }

    // show game log
    let gamelogs = world.fetch::<GameLog>();
    
    let starting_y = 44;
    let max_y = 49;
    for (i, log) in gamelogs.entries.iter().rev().enumerate(){
        if (starting_y + i) < max_y{
            context.print(2, starting_y+i, log);
        }
    }


}