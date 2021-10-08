use bevy::prelude::*;
use rand::Rng;
use crate::{Storable, Interactable, Area, Destructable};

pub struct Tree {
    pub storage: Storable,
}

pub fn generate_forest_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    forest: ResMut<Area>,
) {
    let tree_count = 150;
    let exclude_area = Area::new(Vec2::new(300.0, 300.0));
    let exclude_range_x = exclude_area.x_min..exclude_area.x_max;
    let exclude_range_y = exclude_area.y_min..exclude_area.y_max;

    for _ in 0..tree_count {
        let rand_x = rand::thread_rng().gen_range(forest.x_min..forest.x_max);
        let rand_y = rand::thread_rng().gen_range(forest.y_min..forest.y_max);

        let exclude_x = exclude_range_x.contains(&rand_x);
        let exclude_y = exclude_range_y.contains(&rand_y);

        if (!exclude_x && !exclude_y)
            || (!exclude_x && exclude_y)
            || (!exclude_y && exclude_x)
        {
            let position = Vec2::new(rand_x as f32, rand_y as f32);

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    transform: Transform::from_xyz(position.x, position.y, 0.0),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                })
                .insert(Tree { storage: Storable::new(0.0, 10.0)})
                .insert(Interactable { range: Vec2::new(20.0, 20.0)})
                .insert(Destructable { should_destroy: false });
        }

    }
}