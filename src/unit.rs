use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::{
    BaseLocation, 
    Destroyable, 
    MovementSpeed, 
    Storable, 
    forest_generator::{ExistingTrees, Tree}
};

#[derive(PartialEq)]
pub enum UnitState {
    Idle,
    Walking,
    Carrying,
    Gathering,
    Storing
}

//Idle > Walking > Gathering > Carrying > Storing > Idle

pub struct Unit {
    target_location: Vec3,
    state: UnitState
}

pub struct UnitPlugin;
pub struct MovementTimer(Timer);


impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(MovementTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(initial_unit_system.system())
            .add_system(unit_target_system.system())
            .add_system(unit_resource_collision_system.system())
            // .add_system(unit_movement_system.system())
            .add_system(unit_spawn_system.system());
    }
}

fn init_unit(pos: Vec2, commands: &mut Commands, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .insert(Unit { 
            target_location: Vec3::ZERO,
            state: UnitState::Idle 
        })
        .insert(Storable::new(0.0, 20.0))
        .insert(MovementSpeed(200.))
        .insert(Timer::from_seconds(2., true))
        .insert(Destroyable(false))
        ;
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

pub fn unit_resource_collision_system(
    mut unit_query: Query<(&Sprite, &Transform, &mut Storable, With<Unit>, Without<Tree>)>,
    mut tree_query: Query<(&Sprite, &Transform, &mut Storable, With<Tree>, Without<Unit>)>
) {
    for (unit_sprite, unit_tf, mut unit_store, _, _) in unit_query.iter_mut() 
    {
        let unit_size = unit_sprite.size * Vec2::from(unit_tf.scale.abs());

        for (tree_sprite, tree_tf, mut tree_store, _, _) in tree_query.iter_mut()
        {
            let tree_size = tree_sprite.size * Vec2::from(tree_tf.scale.abs());

            let collision = collide(
                unit_tf.translation,
                unit_size,
                tree_tf.translation,
                tree_size
            );

            println!("Checking collision");

            if let Some(_) = collision {
                let taken = tree_store.modify_by(-(unit_store.max - unit_store.current));
                println!("Took {} from the tree, {} left", taken, tree_store.current);
                unit_store.modify_by(taken);
            }
        }
    }
}

pub fn unit_target_system(
    time: Res<Time>,
    mut unit_query: Query<(&mut Unit, &mut Transform, &mut Timer, &mut Storable)>,
    existing_trees: Res<ExistingTrees>,
    base_location: Res<BaseLocation>
) {
    for (
        mut unit, 
        mut transform, 
        mut timer, 
        mut unit_store
    ) in unit_query.iter_mut() {

        if timer.tick(time.delta()).just_finished() {
            match &unit.state {
                UnitState::Idle => {
                    unit.target_location = find_closest_tree(
                        &transform.translation, 
                        &existing_trees.0
                    );

                    println!("Pos: {}", transform.translation);
                    println!("Closest tree: {}", unit.target_location);

                    unit.state = UnitState::Walking;
                },
                UnitState::Walking => {
                    transform.translation = unit.target_location;

                    unit.state = UnitState::Gathering;
                },
                UnitState::Gathering => {
                    println!("Gathering...");

                    unit.target_location = base_location.0;
                    unit.state = UnitState::Carrying;
                },
                UnitState::Carrying => {
                    transform.translation = unit.target_location;
                    println!("Moving unit to base: {}", unit.target_location);

                    unit.state = UnitState::Storing;
                },
                UnitState::Storing => {
                    println!("Storing...");
                    unit_store.current = 0.0;

                    unit.state = UnitState::Idle;
                }
            }

        }
    }
}

pub fn unit_interaction_system() {
    // Query interactables
    // Find closest interactable
    // interact with interactable
    // Change unitstate
}

pub fn unit_movement_system(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut unit_query: Query<(&mut Unit, &mut Transform, &MovementSpeed)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut unit, mut tf, speed) in unit_query.iter_mut() {

                if unit.state == UnitState::Walking || unit.state == UnitState::Carrying {
                    // let max_distance = time.delta_seconds() * speed.0;
                    // let x_org = tf.translation.x;
                    // let y_org = tf.translation.y;         

                    // let angle = tf.translation.angle_between(unit.target_location);

                    println!("Moving unit to pos {}", unit.target_location);
                    tf.translation = unit.target_location;
                    unit.state = if unit.state == UnitState::Walking {
                        UnitState::Gathering
                    } 
                    else { UnitState::Storing }
                }
        }
    }
}

fn find_closest_tree(pos: &Vec3, trees: &Vec<Tree>) -> Vec3 {
    println!("Treecount: {}", trees.len());
    let first_tree = trees.first();

    match first_tree {
        Some(tree) => {
            let mut shortest_distance = pos.distance(tree.position);
            let mut closest_pos = tree.position;

            for tree in trees.iter() {
                let dist = pos.distance(tree.position);

                if dist < shortest_distance {
                    shortest_distance = dist;
                    closest_pos = tree.position;
                }
            }
            
            closest_pos
        }
        
        None => Vec3::ZERO
        
    }
}