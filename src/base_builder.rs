use bevy::prelude::*;
use crate::{Storable, Interactable};

pub struct Base {
    storage: Storable
}

pub fn spawn_base_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .insert(Base { storage: Storable::new(0.0, 10.0)})
        .insert(Interactable { range: Vec2::new(50.0, 50.0)});
}