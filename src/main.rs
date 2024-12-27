use std::fs::read_to_string;

use bevy::prelude::*;
use render::objects::add_objects;
use render::player::add_player;
use render::puzzle::{setup_puzzle_ticker, smooth_object, step_trigger};
use render::setup_things;
use render::walls::add_walls;
use render::floor::add_floor;
use warehouse::structs::Warehouse;

pub mod warehouse;
pub mod render;

fn main() -> anyhow::Result<()> {
   let text = read_to_string("puzzle.txt")?;
   let warehouse = Warehouse::parse(&text)?;

   App::new()
       .add_plugins(DefaultPlugins)
       .insert_resource(warehouse)
       .add_systems(Startup, (setup_things, add_floor, add_walls, add_objects, add_player, setup_puzzle_ticker))
       .add_systems(Update, (step_trigger, smooth_object))
       .run();

    Ok(())
}
