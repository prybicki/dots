use crate::{MainCamera, MeshHandles};
use bevy::asset::Assets;
use bevy::prelude::shape::Circle;
use bevy::prelude::{Camera2dBundle, Commands, Mesh, ResMut};

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub(crate) fn setup_meshes(mut commands: Commands, mut assets: ResMut<Assets<Mesh>>) {
    let circle = Circle::new(1.0).into();
    let mesh_handles = MeshHandles {
        particle: assets.add(circle).into(),
    };
    commands.insert_resource(mesh_handles);
}
