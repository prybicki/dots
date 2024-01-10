use crate::{
    Border, CameraConfig, Charge, Charges, Dot, DotBundle, EmissionEvent, MainCamera, NNTree,
};
use crate::{MeshHandles, Velocity};
use bevy::asset::Assets;
use bevy::input::Input;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_spatial::kdtree::KDTree2;
use bevy_spatial::SpatialAccess;
use num::clamp;
use std::ops::Mul;

use rand::Rng;

pub fn control_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    cfg: Res<CameraConfig>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    let (mut projection, mut transform) = query.single_mut();
    let displacement = projection.scale * cfg.pan_speed * time.delta_seconds();
    let scale_change = cfg.zoom_speed * time.delta_seconds();
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation -= Vec3::X * displacement;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation += Vec3::X * displacement;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation -= Vec3::Y * displacement;
    }
    if keyboard_input.pressed(KeyCode::W) {
        transform.translation += Vec3::Y * displacement;
    }
    if keyboard_input.pressed(KeyCode::E) {
        projection.scale *= 1.0 - scale_change;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        projection.scale *= 1.0 + scale_change;
    }
}

pub fn control_dots(
    keyboard: Res<Input<KeyCode>>,
    dots: Query<(Entity, With<Velocity>)>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    border: Query<&Border>,
    meshes: Res<MeshHandles>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    if keyboard.just_pressed(KeyCode::F) {
        let area = get_visible_world_rect(camera, camera_transform, window.single());
        let area = border.single().area.intersect(area);
        spawn_particles(area, 1024, &meshes, &mut colors, &mut commands)
    }
    if keyboard.just_pressed(KeyCode::C) {
        for (id, _) in dots.iter() {
            commands.entity(id).despawn()
        }
    }
}

fn get_visible_world_rect(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Rect {
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

    return Rect::from_corners(low_corner_world, high_corner_world);
}

fn spawn_particles(
    area: Rect,
    count: usize,
    meshes: &Res<MeshHandles>,
    colors: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
    let mesh = meshes.particle.clone();
    let mut cs: Vec<Handle<ColorMaterial>> = (0..count)
        .map(|_| colors.add(ColorMaterial::from(Color::WHITE)))
        .collect();
    commands.spawn_batch((0usize..count).map(move |_| {
        let position = Vec2 {
            x: rand::thread_rng().gen_range(area.min.x, area.max.x),
            y: rand::thread_rng().gen_range(area.min.y, area.max.y),
        };
        let transform = Transform::from_translation(Vec3::from(position.extend(0.0)))
            .mul_transform(Transform::from_scale(Vec3::splat(4.0)));
        let velocity = Vec2::new(
            rand::thread_rng().gen_range(-1.0, 1.0),
            rand::thread_rng().gen_range(-1.0, 1.0),
        )
        .mul(Vec2::splat(1.0));
        let rgb = (
            rand::thread_rng().gen_range(0.0, 1.0),
            rand::thread_rng().gen_range(0.0, 1.0),
            rand::thread_rng().gen_range(0.0, 1.0),
        );
        let dot = DotBundle {
            dot: Dot,
            velocity: Velocity { velocity },
            charges: Charges::rgb(rgb),
            cm2d_bundle: ColorMesh2dBundle {
                transform,
                mesh: mesh.clone(),
                material: cs.pop().unwrap(),
                ..default()
            },
        };
        return dot;
    }));
}

pub fn update_colorize(
    objects: Query<(&Charges, &mut Handle<ColorMaterial>)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (charges, obj_mat_handle) in objects.iter() {
        let (r, g, b) = charges.to_rgb();
        let object_material = color_materials.get_mut(obj_mat_handle).unwrap();
        object_material.color = Color::rgb(r, g, b);
    }
}

pub fn update_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, dot) in query.iter_mut() {
        tf.translation += Vec3::from((dot.velocity, 0.0));
    }
}

pub fn update_border(
    border: Query<&Border, Without<Velocity>>,
    mut objects: Query<(Entity, &mut Transform, &mut Velocity)>,
) {
    let rect = border.single().area;
    for (entity, mut tf, mut dot) in objects.iter_mut() {
        if tf.translation.x < rect.min.x {
            tf.translation.x = rect.min.x;
            dot.velocity.x *= -1.0;
        }
        if tf.translation.y < rect.min.y {
            tf.translation.y = rect.min.y;
            dot.velocity.y *= -1.0;
        }
        if tf.translation.x > rect.max.x {
            tf.translation.x = rect.max.x;
            dot.velocity.x *= -1.0;
        }
        if tf.translation.y > rect.max.y {
            tf.translation.y = rect.max.y;
            dot.velocity.y *= -1.0;
        }
    }
}

pub fn update_charge(
    tree: Res<NNTree>,
    mut charges: Query<(&Transform, &mut Charges)>,
    // mut emissions: EventWriter<EmissionEvent>,
) {
    let mut emissions: Vec<(Vec2, Color)> = vec![];
    for (tf, mut charges) in charges.iter_mut() {
        let charges = &mut charges.0;
        for charge in charges {
            charge.inner = clamp(charge.inner + 0.0001, 0.0, 1.0); // Build up
            charge.outer = clamp(charge.outer - 0.05, 0.01, 1.0); // Dissipation
            if charge.inner < 1.0 {
                continue;
            }
            // Emission
            charge.outer = charge.inner;
            charge.inner = 0.0;
            // Propagation
            emissions.push((tf.translation.truncate(), charge.color))
        }
    }

    for (em_pos, em_col) in emissions {
        for (pos, entity) in tree.within_distance(em_pos, 128.0) {
            let (_, mut receiver_charges) = charges.get_mut(entity.unwrap()).unwrap();
            let rec_charges = &mut receiver_charges.0;
            for rec_charge in rec_charges {
                if rec_charge.color == em_col {
                    rec_charge.inner = clamp(rec_charge.inner + 0.01, 0.0, 1.0);
                }
            }
        }
    }
}

// pub fn update_emissions(
//     tree: Res<NNTree>,
//     emissions: EventReader<EmissionEvent>
// ) {
//     for
// }
