use specs::prelude::*;

use specs_derive::Component;
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