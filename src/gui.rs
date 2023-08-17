use specs::{World, WorldExt, Join, Entity};
use rltk::{Rltk, RGB, VirtualKeyCode};

use crate::{CombatStats, Player, GameLog, Name, Position, Map, InBackpack};

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

    // draw mouse
    let mouse_position = context.mouse_pos();
    context.set_bg(mouse_position.0, mouse_position.1, RGB::named(rltk::MAGENTA));

    draw_tooltips(world, context);

}

pub fn draw_tooltips(world: &World, context: &mut Rltk){
    let map = world.fetch::<Map>();
    let names = world.read_storage::<Name>();
    let positions = world.read_storage::<Position>();

    let mouse_position = context.mouse_pos();
    if !Map::is_idx_valid(mouse_position.0, mouse_position.1){
        return;
    }

    let mut tooltip = Vec::new();
    for (name, position) in (&names, &positions).join(){
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_position.0 && position.y == mouse_position.1 && map.currently_visible_tiles[idx]{
            tooltip.push(name.name.to_string());
        }
    }

    if tooltip.is_empty(){
        return;
    }

    let longest_tooltip_item_padding: i32 = tooltip.iter()
        .reduce(|acc, e| {
            if e.len() > acc.len(){
                e
            } else {
                acc
            }
        })
        .unwrap()
        .len() as i32 + 3; // extra padding for longest item

    let arrow_position_x;
    let tooltip_left_x;
    let arrow_string;

    if mouse_position.0 > 40{
        arrow_position_x = mouse_position.0 - 2;
        tooltip_left_x = mouse_position.0 - longest_tooltip_item_padding; // -3 for extra space
        arrow_string = "->";
    } else {
        arrow_position_x = mouse_position.0 + 1;
        tooltip_left_x = mouse_position.0 + 3;
        arrow_string = "<-";
    }

    for (idx, s) in tooltip.iter().enumerate(){
        let y = mouse_position.1 + idx as i32;

        context.print_color(tooltip_left_x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s);

        let current_item_padding = longest_tooltip_item_padding - s.len() as i32 - 1;
        for i in 0..current_item_padding{
            context.print_color(arrow_position_x - i, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &" ".to_string());
        }

        context.print_color(arrow_position_x, mouse_position.1, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), &arrow_string.to_string());
    }
    
}

#[derive(PartialEq, Clone, Copy)]
pub enum ItemMenuResult{
    Exit, 
    NoResponse,
    Selected,
}

pub fn show_inventory(world: &mut World, context: &mut Rltk) -> (ItemMenuResult, Option<Entity>){
    let player_entity = world.fetch::<Entity>();
    let names = world.read_storage::<Name>();
    let backpacks = world.read_storage::<InBackpack>();
    let entities = world.entities();

    let number_of_items = (&backpacks, &names).join()
        .filter(|(backpack, _name)| backpack.owner == *player_entity)
        .count();

    let mut y = (25 - (number_of_items/2)) as i32;

    context.draw_box(15, y-2, 31, number_of_items+3, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    context.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Inventory");
    context.print_color(18, y+number_of_items as i32 + 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Escape to exit");

    let mut item_menu: Vec<Entity> = Vec::new();
    for (j, (name, _backpack, entity)) in (&names, &backpacks, &entities).join().enumerate(){
        context.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        context.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
        context.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        context.print(21, y, &name.name);

        item_menu.push(entity);

        y += 1;
    }

    match context.key{
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key{
            VirtualKeyCode::Escape => (ItemMenuResult::Exit, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if 0 <= selection && selection < item_menu.len() as i32{
                    return (ItemMenuResult::Selected, Some(item_menu[selection as usize]));
                }
                (ItemMenuResult::NoResponse, None)
            },
        }
    }

}

pub fn show_drop_item_menu(world: &mut World, context: &mut Rltk) -> (ItemMenuResult, Option<Entity>){
    let player_entity = world.fetch::<Entity>();
    let names = world.read_storage::<Name>();
    let backpack = world.read_storage::<InBackpack>();
    let entities = world.entities();

    let inventory_count = (&names, &backpack).join()
        .filter(|(_name, pack)| pack.owner == *player_entity)
        .count() as i32;

    let mut y = 25 - (inventory_count/2);

    context.draw_box(15, y-2, 31, inventory_count+3, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    context.print_color(18, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Drop which item?");
    context.print_color(18, y+inventory_count+1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Escape to cancel");

    let mut items: Vec<Entity> = Vec::new();
    for (j, (entity, _backpack, name)) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity).enumerate(){

        context.set(17, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        context.set(18, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as rltk::FontCharType);
        context.set(19, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        context.print(21, y, name.name.to_string());
        
        items.push(entity);

        y+=1;
    }

    match context.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => (ItemMenuResult::Exit, None),
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection >= 0 && selection < inventory_count{
                        return (ItemMenuResult::Selected, Some(items[selection as usize]));
                    } 
                    
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
}