use bevy::prelude::*;
use crate::{BaseLocation, Interactable, Storable};

pub struct Base {
    storage: Storable
}

pub fn spawn_base_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut base_location: ResMut<BaseLocation>
) {
    let pos = Vec3::ZERO;

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .insert(Base { storage: Storable::new(0.0, 100.0)})
        .insert(Interactable { range: Vec2::new(50.0, 50.0)});

    base_location.0 = pos;
}