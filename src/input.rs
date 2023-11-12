use crate::{MainCamera, SpawnParticles};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{
    Camera, EventWriter, GlobalTransform, KeyCode, MouseButton, Query, Res, Window, With,
};
use bevy::window::PrimaryWindow;

pub fn update_input(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    mut spawn: EventWriter<SpawnParticles>,
) {
    let (camera, camera_transform) = camera.single();
    let window = window.single();

    let world_position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    if mouse.pressed(MouseButton::Left) {
        if let Some(p) = world_position {
            spawn.send(SpawnParticles { position: p });
        }
    }
}
