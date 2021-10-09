use bevy::prelude::*;
use crate::{
    Interactable, 
    Storable, 
    base_builder::Base, 
    forest_generator::Tree,
    MovementSpeed
};

pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(spawn_player_system.system())
            .add_system(player_movement_system.system())
            ;
    }
}

pub fn spawn_player_system( 
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>
){
    println!("Spawned player");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .insert(Player )
        .insert(MovementSpeed(500.));
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&MovementSpeed, &mut Transform, With<Player>)>
) { 
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        
        let dir = 
        if keyboard_input.pressed(KeyCode::Left) {
            Vec3::new(-1., 0.,0.)
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            Vec3::new(1., 0., 0.)
        }
        else if keyboard_input.pressed(KeyCode::Up) {
            Vec3::new(0., 1., 0.)
        }
        else if keyboard_input.pressed(KeyCode::Down) {
            Vec3::new(0., -1., 0.)
        }
        else {
            Vec3::ZERO
        };

        transform.translation.x += dir.x * speed.0 * time.delta_seconds();
        transform.translation.y += dir.y * speed.0 * time.delta_seconds();
    }
}