use bevy::prelude::*;
use crate::{
    Interactable, 
    Storable, 
    base_builder::Base, 
    forest_generator::Tree,
    MovementSpeed
};

pub enum UnitState {
    Idle,
    Walking,
    Carrying,
    Gathering
}

pub struct Unit {
    walking_speed: f32,
    target_location: Vec2,
}

fn init_unit(pos: Vec2, commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .insert(Unit { walking_speed: 200.0, target_location: Vec2::ZERO })
        .insert(UnitState::Idle)
        .insert(Storable::new(0.0, 5.0));
}

pub fn initial_unit_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let pos = Vec2::new(0.0, 100.0);
    init_unit(pos, &mut commands, &mut materials)
}

pub fn unit_spawn_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
   if keyboard_input.just_pressed(KeyCode::Space) {
        let pos = Vec2::new(100.0, 0.0 );
        init_unit(pos, &mut commands, &mut materials)
   } 
}

pub fn unit_target_system(
    mut commands: Commands,
    mut tree_query: Query<&Tree>
) {
   if let Ok((unit)) = tree_query.single_mut() {
        println!("{:?}", unit.storage.max);
   }
   else {
       println!("Oeps");
   }
}