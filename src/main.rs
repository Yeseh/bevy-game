extern crate rand;

mod forest_generator;
mod base_builder;
mod unit;
mod player;

use forest_generator::ForestPlugin;
use base_builder::{spawn_base_system};
use unit::UnitPlugin;
use player::PlayerPlugin;
use bevy::prelude::*;

pub struct Destroyable(bool);
pub struct MovementSpeed(f32);
pub struct BaseLocation(Vec3);

pub struct Storage(f32);

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

#[derive(Debug, Copy, Clone)]
pub struct Storable {
    pub current: f32,
    pub max: f32,
    pub min: f32
}

impl Storable {
    pub fn new(min: f32, max: f32) -> Storable {
        Storable {
            current: max,
            max: max,
            min: min
        }
    }

    pub fn is_empty(&self) -> bool {
        self.current <= self.min
    }

    // Modifies by amount, returns the actual applied modification
    pub fn modify_by(&mut self, amount: f32) -> f32 { 
        let mut diff = self.current + amount;
        let clamp = diff.clamp(self.min, self.max);

        if clamp == self.max {
            diff = self.max - self.current;
        }
        else if clamp == self.min {
            diff = self.min - self.current
        }

        self.current = clamp; 

        diff
    }
}

pub fn despawn_destroyed_entity_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Destroyable)>
) {
    for (entity, destroyed) in query.iter_mut() {
        if destroyed.0 {
            commands.entity(entity).despawn();
        }
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
        .insert_resource(BaseLocation(Vec3::ZERO))
        .add_system(despawn_destroyed_entity_system.system())
        .add_startup_system(setup.system())
        .add_startup_system(spawn_base_system.system())
        .add_plugin(PlayerPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(ForestPlugin)
        .run();
}