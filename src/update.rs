use crate::{CameraConfig, MainCamera};
use crate::{Dot, MeshHandles};
use bevy::asset::Assets;
use bevy::input::Input;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_spatial::kdtree::KDTree2;

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
    dots: Query<(Entity, With<Dot>)>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    meshes: Res<MeshHandles>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    if keyboard.just_pressed(KeyCode::F) {
        let area = get_visible_world_rect(camera, camera_transform, window.single());
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

type NNTree = KDTree2<Dot>;

pub fn update_colorize(
    time: Res<Time>,
    objects: Query<(&mut Transform, &mut Handle<ColorMaterial>)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    for (tf, obj_mat_handle) in objects.iter() {
        let (r, g, b) = (
            (tf.translation.x / 256.0).sin().abs(),
            (tf.translation.y / 256.0).sin().abs(),
            time.elapsed_seconds().sin().abs(),
        );
        let object_material = color_materials.get_mut(obj_mat_handle).unwrap();
        object_material.color = Color::rgb(r, g, b);
    }
}

pub fn update_dots(_tree: Res<NNTree>, mut query: Query<(&mut Transform, &Dot)>) {
    for (mut tf, dot) in query.iter_mut() {
        tf.translation += Vec3::from((dot.velocity, 0.0));
    }
}
