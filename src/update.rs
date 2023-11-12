use crate::{Dot, MeshHandles, SpawnParticles};
use bevy::asset::Assets;
use bevy::ecs::entity::Entities;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Color, ColorMaterial, Commands, EventReader, Query, Res, ResMut, Transform,
};
use bevy::sprite::MaterialMesh2dBundle;
use rand::{random, Rng};

pub fn update_spawn(
    mut commands: Commands,
    mut events: EventReader<SpawnParticles>,
    meshes: ResMut<MeshHandles>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    for event in events.iter() {
        // TODO: inefficient!
        let random_color = Color::rgb(random(), random(), random());
        let mut orig_bundle = MaterialMesh2dBundle {
            mesh: meshes.particle.clone(),
            material: colors.add(ColorMaterial::from(random_color)),
            ..default()
        };
        let scale = Vec3::splat(5.0);
        let tf = Transform::from_translation(Vec3::from((event.position, 0.0)))
            .mul_transform(Transform::from_scale(scale));

        let iter = (0..16).map(move |_| {
            let velocity = Vec2::new(
                rand::thread_rng().gen_range(-1.0, 1.0),
                rand::thread_rng().gen_range(-1.0, 1.0),
            );
            let mut bundle = MaterialMesh2dBundle {
                transform: tf,
                ..orig_bundle.clone()
            };
            return (bundle, Dot { velocity });
        });
        commands.spawn_batch(iter);
    }
}

pub fn update_dots(mut query: Query<(&mut Transform, &Dot)>) {
    for (mut tf, dot) in query.iter_mut() {
        tf.translation += Vec3::from((dot.velocity, 0.0));
    }
}

pub fn update_print_entity_count(e: &Entities) {
    println!("Entity count: {}", e.len());
}
