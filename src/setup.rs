use crate::{Border, CameraConfig, MainCamera, MeshHandles};
use bevy::asset::Assets;
use bevy::math::Rect;
use bevy::prelude::shape::Circle;
use bevy::prelude::{Camera2dBundle, Commands, Mesh, ResMut, Transform, Vec2};
use bevy::utils::default;

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera));
    commands.insert_resource(CameraConfig {
        pan_speed: 512.0,
        zoom_speed: 1.0,
    });
}

pub(crate) fn setup_meshes(mut commands: Commands, mut assets: ResMut<Assets<Mesh>>) {
    let circle = Circle::new(1.0).into();
    let mesh_handles = MeshHandles {
        particle: assets.add(circle).into(),
    };
    commands.insert_resource(mesh_handles);
}

pub(crate) fn setup_environment(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        Border {
            area: Rect::from_corners(Vec2::new(-1920.0, -1080.0), Vec2::new(1920.0, 1080.0)),
        },
    ));
}
