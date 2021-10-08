extern crate rand;

mod forest_generator;
mod base_builder;
mod unit;
mod player;

use forest_generator::{generate_forest_system};
use base_builder::{spawn_base_system};
use unit::{initial_unit_system, unit_spawn_system, unit_target_system};
use player::{spawn_player_system, player_movement_system};
use bevy::prelude::*;

pub struct Destructable {
    pub should_destroy: bool
}

pub struct MovementSpeed(f32);

struct WinSize {
    w: f32,
    h: f32
}

pub struct Area {
   pub x_max: f32,
   pub x_min: f32,
   pub y_max: f32,
   pub y_min: f32
}

impl Area {
    fn new(bounds: Vec2) -> Area {
        Area {
            x_max: bounds.x / 2.0,
            x_min: -bounds.x / 2.0,
            y_max: bounds.y / 2.0,
            y_min: -bounds.y / 2.0
        }
    }
}

pub struct Interactable {
    range: Vec2
}

// Component
pub struct Storable {
    pub current: f32,
    pub max: f32,
    pub min: f32
}

impl Storable {
    fn new(min: f32, max: f32) -> Storable {
        Storable {
            current: max,
            max: max,
            min: min
        }
    }

    fn modify_by(&mut self, amount: f32) { 
        self.current = (self.current + amount).clamp(self.min, self.max);
    }
}


/*
enum DroneState {
    Gathering,
    Carrying,
    Moving,
    Idle
}

// Component
struct Drone {
    state: DroneState,
    speed: f32
}
*/

fn setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut windows: ResMut<Windows>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    
    let mut window = windows.get_primary_mut().unwrap();

    window.set_position(IVec2::new(1280, 720));

    commands.insert_resource(WinSize { 
        w: window.width(), 
        h: window.height() 
    });
}

fn main() {


    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9,0.9,0.9)))
        .insert_resource(Area::new(Vec2::new(1280.0, 720.0)))
        .add_startup_system(setup.system())
        .add_startup_system(spawn_base_system.system())
        .add_startup_system(initial_unit_system.system())
        .add_startup_system(generate_forest_system.system())
        .add_startup_system(spawn_player_system.system())
        .add_system(unit_spawn_system.system())
        // .add_system(unit_target_system.system())
        .add_system(player_movement_system.system())
        .run();
}