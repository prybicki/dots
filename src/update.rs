use crate::{ClearSimulation, MainCamera};
use crate::{Dot, MeshHandles, SpawnParticles};
use bevy::asset::Assets;
use bevy::ecs::entity::Entities;
use bevy::input::Input;
use bevy::math::{Vec2, Vec3, Vec3Swizzles};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_spatial::kdtree::KDTree2;
use bevy_spatial::SpatialAccess;
use std::ops::Deref;

use rand::Rng;

pub fn update_input(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    _mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut clear_simulation: EventWriter<ClearSimulation>,
    mut spawn_particles: EventWriter<SpawnParticles>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    let _mouse_world_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    let low_corner_world = camera
        .viewport_to_world(camera_transform, Vec2::new(0.0, 0.0))
        .unwrap()
        .origin
        .truncate();
    let high_corner_world = camera
        .viewport_to_world(
            camera_transform,
            Vec2::new(window.resolution.width(), window.resolution.height()),
        )
        .unwrap()
        .origin
        .truncate();

    if keyboard.just_pressed(KeyCode::C) {
        clear_simulation.send(ClearSimulation);
    }

    if keyboard.just_pressed(KeyCode::R) {
        spawn_particles.send(SpawnParticles {
            area: Rect::from_corners(low_corner_world, high_corner_world),
            count: 1024,
        });
    }
}

pub fn on_clear_simulation_event(
    mut commands: Commands,
    dots: Query<(Entity, With<Dot>)>,
    mut clear_simulation: EventReader<ClearSimulation>,
) {
    for _ in clear_simulation.iter() {
        for (id, _) in dots.iter() {
            commands.entity(id).despawn()
        }
    }
}

pub fn on_spawn_particles_event(
    mut commands: Commands,
    mut spawn_particles: EventReader<SpawnParticles>,
    meshes: ResMut<MeshHandles>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    for event in spawn_particles.iter() {
        let min = event.area.min;
        let max = event.area.max;
        let mesh = meshes.particle.clone();
        let mut cs: Vec<Handle<ColorMaterial>> = (0..event.count)
            .map(|_| colors.add(ColorMaterial::from(Color::WHITE)))
            .collect();
        commands.spawn_batch((0usize..event.count).map(move |i| {
            let position = Vec2 {
                x: rand::thread_rng().gen_range(min.x, max.x),
                y: rand::thread_rng().gen_range(min.y, max.y),
            };
            let transform = Transform::from_translation(Vec3::from(position.extend(0.0)))
                .mul_transform(Transform::from_scale(Vec3::splat(4.0)));
            let velocity = Vec2::new(
                rand::thread_rng().gen_range(-1.0, 1.0),
                rand::thread_rng().gen_range(-1.0, 1.0),
            );
            let bundle = MaterialMesh2dBundle {
                transform,
                mesh: mesh.clone(),
                material: cs.pop().unwrap(),
                ..default()
            };
            return (bundle, Dot { velocity });
        }));
    }
}

type NNTree = KDTree2<Dot>;

pub fn update_dots(
    tree: Res<NNTree>,
    mut query: Query<(&mut Transform, &Dot, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut tf, dot, mut mat) in query.iter_mut() {
        tf.translation += Vec3::from((dot.velocity, 0.0));
        // let neighbours = tree.within_distance(tf.translation.xy(), 28.0).len();
        // TODO: bug, this is not working
        if let Some(neighbour) = tree.nearest_neighbour(tf.translation.xy()) {
            let diff_x = tf.translation.x - neighbour.0.x;
            let diff_y = tf.translation.y - neighbour.0.y;
            tf.scale = Vec2::new(diff_x, diff_y).extend(0.0);
            // if let Some(material) = materials.get_mut(&mat) {
            //     material.color = Color::rgb(diff_x.sin().abs(), diff_y.sin().abs(), 0.5);
            //     // Change color here
            // }
        }
    }
}

pub fn update_print_entity_count(e: &Entities) {
    println!("Entity count: {}", e.len());
}
