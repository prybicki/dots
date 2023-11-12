mod input;
mod setup;
mod update;

use bevy::asset::Asset;
use bevy::ecs::entity::Entities;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::WindowResolution;
use rand::{random, Rng};

use input::*;
use setup::*;
use update::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct MeshHandles {
    particle: Mesh2dHandle,
}

#[derive(Event)]
pub struct SpawnParticles {
    position: Vec2,
}

#[derive(Component)]
pub struct Dot {
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1600.0, 900.0),
                ..default()
            }),
            ..default()
        }))
        .add_event::<SpawnParticles>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_meshes)
        .add_systems(Update, update_input)
        .add_systems(Update, update_spawn)
        .add_systems(Update, update_dots)
        .add_systems(Update, update_print_entity_count)
        .run();
}

/** UPDATE **/

fn spawn_dots(
    mut commands: Commands,
    entities: &Entities,
    meshes: Res<MeshHandles>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    if entities.len() >= 10000 {
        return;
    }
    let xyz = Vec3::new(
        rand::thread_rng().gen_range(-800.0, 800.0),
        rand::thread_rng().gen_range(-450.0, 450.0),
        0.0,
    );

    let scale = Vec3::splat(5.0);
    let translation = xyz;
    let tf = Transform::from_translation(translation).mul_transform(Transform::from_scale(scale));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.particle.clone(),
        material: colors.add(ColorMaterial::from(Color::WHITE)),
        transform: tf,
        ..default()
    });
}

// fn update(query: Query<(Entity,)>) {}
fn update(mut local: Local<i32>, mut local2: Local<i32>) {
    *local += 1;
    println!("local: {}", *local);
    println!("local2: {}", *local2);
}

fn gizmos(mut gizmos: Gizmos) {}
