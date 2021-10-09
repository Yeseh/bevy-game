use bevy::prelude::*;
use rand::Rng;
use crate::{Area, Destroyable, Interactable, Storable};

#[derive(Debug, Copy, Clone)]
pub struct Tree {
    pub storage: Storable,
    pub position: Vec3,
}

pub struct ForestPlugin;

impl Plugin for ForestPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(ExistingTrees(vec![]))
            .add_startup_system(generate_forest_system.system())
            .add_system(deforestation_system.system())
            ;
    }
}

pub struct ExistingTrees(pub Vec<Tree>);

pub fn deforestation_system(
    mut query: Query<(&Tree, &Transform, &mut Destroyable)>,
    mut existing_trees: ResMut<ExistingTrees>
) {
    for (tree, transform, mut destroyed) in query.iter_mut() {
        if tree.storage.is_empty() {
            destroyed.0 = true;

            for i in 0..existing_trees.0.iter().len() {
                if existing_trees.0[i].position.eq(&transform.translation) {
                    existing_trees.0.remove(i);
                }
            }
        }
    }
}
pub fn generate_forest_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut existing_trees: ResMut<ExistingTrees>,
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
            let position = Vec3::new(rand_x as f32, rand_y as f32, 0.0);
            let tree = Tree {
                storage: Storable::new(0.0, 10.0), 
                position
            };

            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    transform: Transform::from_xyz(position.x, position.y, position.z),
                    sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                })
                .insert(tree)
                .insert(Interactable { range: Vec2::new(20.0, 20.0)})
                .insert(Destroyable(false));

            existing_trees.0.push(tree);
        }

    }
}