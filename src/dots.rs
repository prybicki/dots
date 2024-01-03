use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::window::WindowResolution;
use bevy_spatial::*;
use std::time::Duration;

mod input;
mod setup;
mod update;

use setup::*;
use update::*;

/** RESOURCES **/

#[derive(Resource)]
pub struct MeshHandles {
    particle: Mesh2dHandle,
}

/** EVENTS **/

#[derive(Event)]
pub struct ClearSimulation;

#[derive(Event)]
pub struct SpawnParticles {
    area: Rect,
    count: usize,
}

/** COMPONENTS **/

#[derive(Component)]
pub struct MainCamera;

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
        .add_plugins(
            AutomaticUpdate::<Dot>::new()
                .with_frequency(Duration::from_millis(0))
                .with_spatial_ds(SpatialStructure::KDTree2),
        )
        .add_event::<SpawnParticles>()
        .add_event::<ClearSimulation>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_meshes)
        .add_systems(Update, update_input)
        .add_systems(Update, on_spawn_particles_event)
        .add_systems(Update, on_clear_simulation_event)
        .add_systems(Update, update_dots)
        .add_systems(Update, update_print_entity_count)
        .run();
}
