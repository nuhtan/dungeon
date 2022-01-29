use std::{fs::File, io::Write};

use bevy::prelude::*;

mod floor;

use floor::Floor;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    DownLeft,
    UpRight,
    DownRight
}

fn main() {
    let floor_dims = (90, 30);
    let floor = Floor::gen_floor(floor_dims, 3..8, 6..15);
    for y in 0..floor_dims.1 {
        let mut line = "".to_string();
        for x in 0..floor_dims.0 {
            line.push( match floor.point_in_room((x, y)) {
                true => ' ',
                false => 'O'
            });
            
        }
        println!("{}", line);
    }
    // App::build()
    //     .add_plugins(DefaultPlugins)
    //     .add_startup_system(setup.system())
    //     .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    

}

fn generate_floor() {

}